#!/usr/bin/env rust-script
//! Local writing and publishing commands for Software Notes.

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const HELP: &str = "Usage:
  rust-script _blog.rs new \"Post title\"
  rust-script _blog.rs build
  rust-script _blog.rs publish _posts/YYYY-MM-DD-slug.md

new creates a dated Markdown file in _posts. Edit it before publishing.
build renders the local site into _site.
publish builds, commits the selected post, and pushes main to origin.
GitHub Pages deploys the push to https://rixrix.github.io/.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Blog: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let script =
        PathBuf::from(env::var_os("RUST_SCRIPT_PATH").ok_or("Run this with rust-script.")?);
    env::set_current_dir(script.parent().ok_or("Cannot locate the blog folder.")?)?;

    match args.as_slice() {
        [] => println!("{HELP}"),
        [flag] if ["help", "--help", "-h"].contains(&flag.as_str()) => println!("{HELP}"),
        [action, title] if action == "new" => new_post(title)?,
        [action] if action == "build" => build()?,
        [action, path] if action == "publish" => publish(path)?,
        _ => return Err(HELP.into()),
    }
    Ok(())
}

fn new_post(title: &str) -> Result<()> {
    let title = title.trim();
    if title.is_empty() || title.chars().any(char::is_control) {
        return Err("Use a nonempty title on one line.".into());
    }
    let slug = title
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .to_ascii_lowercase();
    if slug.is_empty() {
        return Err("Use at least one ASCII letter or number in the title.".into());
    }
    let date = output(
        Command::new("date")
            .env("TZ", "Pacific/Auckland")
            .arg("+%Y-%m-%d %H:%M:%S %z"),
    )?;
    let path = format!("_posts/{}-{slug}.md", &date[..10]);
    let title = title.replace('\'', "''");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|error| {
            if error.kind() == io::ErrorKind::AlreadyExists {
                io::Error::new(error.kind(), format!("{path} already exists"))
            } else {
                error
            }
        })?;
    writeln!(
        file,
        "---\ntitle: '{title}'\ndate: {date}\ncategory: Field notes\ntags: []\ndescription: ''\n---\n"
    )?;
    println!("Created {path}");
    Ok(())
}

fn build() -> Result<()> {
    let mut jekyll = if Path::new(".jekyll-gems/bin/jekyll").is_file() {
        let mut command = Command::new(".jekyll-gems/bin/jekyll");
        command.env("GEM_HOME", ".jekyll-gems");
        command
    } else {
        Command::new("jekyll")
    };
    checked(jekyll.arg("build"))
}

fn publish(path: &str) -> Result<()> {
    let post = Path::new(path);
    if post.parent() != Some(Path::new("_posts"))
        || post.extension().and_then(|extension| extension.to_str()) != Some("md")
        || !fs::symlink_metadata(post)?.is_file()
    {
        return Err("Select a Markdown file directly inside _posts.".into());
    }
    let contents = fs::read_to_string(post)?;
    let mut lines = contents.lines();
    let has_front_matter = lines.next() == Some("---") && lines.any(|line| line == "---");
    if !has_front_matter || !lines.any(|line| !line.trim().is_empty()) {
        return Err("Add front matter and write the post before publishing.".into());
    }
    if output(Command::new("git").args(["branch", "--show-current"]))? != "main" {
        return Err("Switch to main before publishing.".into());
    }

    checked(Command::new("git").args(["fetch", "origin", "main"]))?;
    checked(Command::new("git").args(["merge-base", "--is-ancestor", "origin/main", "HEAD"]))?;
    let pending = output(Command::new("git").args(["diff", "--name-only", "origin/main", "HEAD"]))?;
    if pending.lines().any(|file| file != path) {
        return Err(
            "main has unpushed changes outside the selected post. Review them before publishing."
                .into(),
        );
    }

    build()?;
    let changed = output(Command::new("git").args(["status", "--porcelain", "--", path]))?;
    if !changed.is_empty() {
        checked(Command::new("git").args(["add", "--", path]))?;
        checked(Command::new("git").args([
            "commit",
            "--only",
            "-m",
            "Publish blog post",
            "--",
            path,
        ]))?;
    }
    checked(Command::new("git").args(["push", "origin", "HEAD:main"]))?;
    println!("Pushed to GitHub. Check https://rixrix.github.io/ after Pages deploys.");
    Ok(())
}

fn checked(command: &mut Command) -> Result<()> {
    let status = command
        .status()
        .map_err(|error| format!("{command:?}: {error}"))?;
    if !status.success() {
        return Err(format!("{command:?} exited with {status}").into());
    }
    Ok(())
}

fn output(command: &mut Command) -> Result<String> {
    let result = command.output()?;
    if !result.status.success() {
        return Err(format!("{command:?}: {}", String::from_utf8_lossy(&result.stderr)).into());
    }
    Ok(String::from_utf8(result.stdout)?.trim().to_owned())
}
