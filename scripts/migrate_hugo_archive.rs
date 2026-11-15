use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let old_repo = arguments
        .first()
        .ok_or("Usage: migrate_hugo_archive <old-repository> [--overwrite]")?;
    let overwrite = arguments.iter().any(|argument| argument == "--overwrite");
    let files = command_output(
        "git",
        &[
            "-C",
            &old_repo,
            "ls-tree",
            "-r",
            "--name-only",
            "origin/hugo-cms",
            "content/post",
        ],
    )?;

    let mut migrated = 0;
    for source in files.lines().filter(|path| path.ends_with(".md")) {
        let raw = command_output(
            "git",
            &[
                "-C",
                &old_repo,
                "show",
                &format!("origin/hugo-cms:{source}"),
            ],
        )?;
        let post = parse_post(&raw)?;
        let filename = format!("{}-{}.md", &post.date[..10], slugify(&post.title));
        let destination = Path::new("_posts").join(filename);
        if destination.exists() && !overwrite {
            return Err(format!("Refusing to overwrite {}", destination.display()).into());
        }

        let tags = tags_for(&post.tags, &post.categories, &post.title, &post.body);
        let tags = tags.join(", ");
        let title = post.title.replace('\'', "''");
        let body = convert_galleries(&post.body)
            .replace(
                "http://mindginative.com/post/asrock-deskmini-110w-with-ubuntu-server/",
                "/notes/2017/05/11/asrock-h110m-stx-deskmini-110w-with-headless-ubuntu-server/",
            )
            .replace(
                "http://yup-the-website-domain-is.mindginative.com/post/asrock-deskmini-110w-with-ubuntu-server/",
                "/notes/2017/05/11/asrock-h110m-stx-deskmini-110w-with-headless-ubuntu-server/",
            );
        let content = format!(
            "---\ntitle: '{title}'\ndate: {}\ncategory: Field notes\ntags: [{tags}]\ndescription: ''\n---\n\n{}\n",
            post.date,
            body
        );
        fs::write(&destination, content)?;
        println!("{} -> {}", source, destination.display());
        migrated += 1;
    }

    println!("Migrated {migrated} historical posts");
    Ok(())
}

struct Post {
    title: String,
    date: String,
    tags: Vec<String>,
    categories: Vec<String>,
    body: String,
}

fn parse_post(raw: &str) -> Result<Post> {
    let mut lines = raw.lines();
    let delimiter = lines.next().ok_or("Post is empty")?.to_owned();
    if delimiter != "+++" && delimiter != "---" {
        return Err("Post has no supported front matter delimiter".into());
    }

    let mut front_matter = String::new();
    let mut body = String::new();
    let mut in_front_matter = true;
    for line in lines {
        if in_front_matter && line == delimiter {
            in_front_matter = false;
            continue;
        }
        if in_front_matter {
            front_matter.push_str(line);
            front_matter.push('\n');
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }

    let title = field(&front_matter, "title").ok_or("Post has no title")?;
    let date = field(&front_matter, "date").ok_or("Post has no date")?;
    Ok(Post {
        title,
        date,
        tags: array_field(&front_matter, "tags"),
        categories: array_field(&front_matter, "categories"),
        body: body.trim().to_owned(),
    })
}

fn field(front_matter: &str, key: &str) -> Option<String> {
    front_matter.lines().find_map(|line| {
        let (name, value) = line.split_once('=').or_else(|| line.split_once(':'))?;
        if name.trim() != key {
            return None;
        }
        Some(value.trim().trim_matches('"').trim_matches('\'').to_owned())
    })
}

fn array_field(front_matter: &str, key: &str) -> Vec<String> {
    let Some(start) = front_matter.find(key) else {
        return Vec::new();
    };
    let rest = &front_matter[start..];
    let Some(open) = rest.find('[') else {
        return Vec::new();
    };
    let Some(close) = rest[open..].find(']') else {
        return Vec::new();
    };
    let value = &rest[open + 1..open + close];
    quoted_values(value)
}

fn quoted_values(value: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut quote = false;
    for character in value.chars() {
        if character == '"' {
            if quote && !current.is_empty() {
                values.push(current.clone());
                current.clear();
            }
            quote = !quote;
        } else if quote {
            current.push(character);
        }
    }
    values
}

fn tags_for(source_tags: &[String], categories: &[String], title: &str, body: &str) -> Vec<String> {
    let mut tags = Vec::new();
    for value in source_tags.iter().chain(categories.iter()) {
        let tag = normalize_tag(value);
        if !tag.is_empty() && tag != "blog" && !tags.contains(&tag) {
            tags.push(tag);
        }
    }

    let haystack = format!("{} {}", title, body).to_ascii_lowercase();
    if tags.is_empty() {
        let title_lower = title.to_ascii_lowercase();
        if title_lower.contains("resolution") {
            tags.extend(["life".to_owned(), "goals".to_owned()]);
        } else if title_lower.contains("moving to hugo") {
            tags.extend(["blogging".to_owned(), "hugo".to_owned()]);
        }
    }

    let inferred = [
        ("rust", "rust"),
        ("typescript", "typescript"),
        ("javascript", "javascript"),
        ("nodejs", "nodejs"),
        ("docker", "docker"),
        ("aws", "aws"),
        ("ubuntu", "ubuntu"),
        ("windows", "windows"),
        ("hugo", "hugo"),
        ("xhyve", "virtualization"),
        ("alpine", "alpine-linux"),
        ("react", "react"),
    ];
    if tags.is_empty() {
        for (needle, tag) in inferred {
            if haystack.contains(needle) && !tags.iter().any(|existing| existing == tag) {
                tags.push(tag.to_owned());
            }
        }
    }
    if tags.is_empty() {
        tags.push("notes".to_owned());
    }
    tags.sort();
    tags
}

fn normalize_tag(value: &str) -> String {
    let lower = value.trim().to_ascii_lowercase();
    match lower.as_str() {
        "windows10" => "windows-10".to_owned(),
        "win10" => "windows-10".to_owned(),
        "raspberry" => "raspberry-pi".to_owned(),
        other => other.replace('_', "-").replace(' ', "-"),
    }
}

fn convert_galleries(body: &str) -> String {
    let mut output = String::new();
    let mut in_gallery = false;
    let mut figures = Vec::new();

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("{{< gallery") {
            in_gallery = true;
            figures.clear();
            continue;
        }
        if in_gallery && trimmed == ">}}" {
            output.push_str("<div class=\"legacy-gallery\">\n");
            for (path, caption) in figures.drain(..) {
                output.push_str(&format!(
                    "<figure><img src=\"/images/{path}\" alt=\"{caption}\"><figcaption>{caption}</figcaption></figure>\n"
                ));
            }
            output.push_str("</div>\n\n");
            in_gallery = false;
            continue;
        }
        if in_gallery {
            let value = trimmed.trim_matches('"');
            if let Some((path, caption)) = value.split_once('|') {
                figures.push((path.to_owned(), escape_html(caption)));
            }
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }
    output.trim().to_owned()
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut separator = false;
    for character in title.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character.to_ascii_lowercase());
            separator = false;
        } else if !slug.is_empty() {
            separator = true;
        }
    }
    slug
}

fn command_output(command: &str, arguments: &[&str]) -> Result<String> {
    let output = Command::new(command).args(arguments).output()?;
    if !output.status.success() {
        return Err(format!(
            "{command} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}
