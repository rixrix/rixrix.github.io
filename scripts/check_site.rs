#!/usr/bin/env rust-script
//! Verify the generated blog’s internal links, assets, anchors, and RSS feed.
//!
//! ```cargo
//! [dependencies]
//! html5ever = "=0.39.0"
//! markup5ever_rcdom = "=0.39.0"
//! percent-encoding = "=2.3.2"
//! roxmltree = "=0.21.1"
//! url = "=2.5.8"
//! walkdir = "=2.5.0"
//! ```

use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use percent_encoding::percent_decode_str;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
struct Page {
    anchors: HashSet<String>,
    references: Vec<String>,
}

impl Page {
    fn collect(&mut self, node: &Handle) {
        if let NodeData::Element { name, attrs, .. } = &node.data {
            for attr in attrs.borrow().iter() {
                let value = attr.value.as_ref();
                if value.is_empty() {
                    continue;
                }
                match attr.name.local.as_ref() {
                    "id" => {
                        self.anchors.insert(value.to_owned());
                    }
                    "name" if name.local.as_ref() == "a" => {
                        self.anchors.insert(value.to_owned());
                    }
                    "href" | "src" | "poster" => self.references.push(value.to_owned()),
                    _ => {}
                }
            }
        }
        for child in node.children.borrow().iter() {
            self.collect(child);
        }
    }
}

fn main() -> Result<()> {
    let root = PathBuf::from(
        env::args_os()
            .nth(1)
            .ok_or("Usage: rust-script scripts/check_site.rs _site")?,
    );
    let origin = Url::parse("https://rixrix.github.io/")?;
    let mut pages = BTreeMap::new();
    for entry in WalkDir::new(&root) {
        let entry = entry?;
        let path = entry.path();
        if !entry.file_type().is_file()
            || path.extension().and_then(|ext| ext.to_str()) != Some("html")
        {
            continue;
        }
        let html = fs::read_to_string(path)?;
        let dom = parse_document(RcDom::default(), Default::default()).one(html);
        let mut page = Page::default();
        page.collect(&dom.document);
        pages.insert(path.to_path_buf(), page);
    }

    let feed = fs::read_to_string(root.join("feed.xml"))?;
    let feed = roxmltree::Document::parse(&feed)?;
    let home = pages
        .get_mut(&root.join("index.html"))
        .ok_or("Generated homepage is missing")?;
    if !feed.root_element().has_tag_name("rss") {
        return Err("Expected an RSS feed".into());
    }
    for item in feed.descendants().filter(|node| node.has_tag_name("item")) {
        let link = item
            .children()
            .find(|node| node.has_tag_name("link"))
            .and_then(|node| node.text())
            .ok_or("RSS item is missing its link")?;
        home.references.push(link.to_owned());
    }

    let mut errors = BTreeSet::new();
    for (path, page) in &pages {
        let source = path.strip_prefix(&root)?.to_string_lossy();
        let route = source.strip_suffix("index.html").unwrap_or(&source);
        let base = origin.join(route)?;
        for reference in &page.references {
            let Ok(target) = base.join(reference) else {
                errors.insert(format!("{source}: invalid URL {reference}"));
                continue;
            };
            if !matches!(target.scheme(), "http" | "https")
                || target.host_str() != origin.host_str()
            {
                continue;
            }
            let target_path = local_target(&root, &target)?;
            if !target_path.is_file() {
                errors.insert(format!("{source}: missing target {reference}"));
                continue;
            }
            if let (Some(target_page), Some(fragment)) =
                (pages.get(&target_path), target.fragment())
            {
                let anchor = percent_decode_str(fragment).decode_utf8()?;
                if !anchor.is_empty() && !target_page.anchors.contains(anchor.as_ref()) {
                    errors.insert(format!("{source}: missing anchor {reference}"));
                }
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors.into_iter().collect::<Vec<_>>().join("\n").into());
    }
    println!(
        "Checked {} generated pages, internal references, and RSS.",
        pages.len()
    );
    Ok(())
}

fn local_target(root: &Path, target: &Url) -> Result<PathBuf> {
    let decoded = percent_decode_str(target.path()).decode_utf8()?;
    let mut path = root.join(decoded.trim_start_matches('/'));
    if path.is_dir() {
        path = path.join("index.html");
    } else if !path.is_file() && path.extension().is_none() && path.with_extension("html").is_file()
    {
        path.set_extension("html");
    }
    Ok(path)
}
