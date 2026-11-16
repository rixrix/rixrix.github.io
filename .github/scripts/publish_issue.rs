use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const TAG_RULES: &[(&str, &[&str])] = &[
    ("rust", &["rust", "rustc", "cargo", "rust lang"]),
    ("github-actions", &["github actions", "github workflow"]),
    (
        "spec-driven-development",
        &["spec driven development", "spec driven", "spec kit"],
    ),
    (
        "ai-agents",
        &[
            "ai agents",
            "ai coding agents",
            "ai assisted coding",
            "agenticflowx",
        ],
    ),
    ("typescript", &["typescript", "tsc"]),
    ("javascript", &["javascript", "ecmascript"]),
    ("nodejs", &["node js", "nodejs"]),
    ("raspberry-pi", &["raspberry pi", "raspbian"]),
    (
        "audio",
        &["audio", "guitar processor", "neural amp modeler"],
    ),
    ("midi", &["midi"]),
    ("docker", &["docker", "dockerfile"]),
    ("kubernetes", &["kubernetes", "k8s"]),
    ("linux", &["linux"]),
    ("aws", &["aws", "amazon web services"]),
    ("azure", &["azure"]),
    ("jekyll", &["jekyll"]),
    ("markdown", &["markdown"]),
    ("deployment", &["deployment", "deploying"]),
];

fn main() -> Result<()> {
    let title = env::var("ISSUE_TITLE")?.trim().to_owned();
    let issue_body = env::var("ISSUE_BODY")?;
    let (explicit_tags, body) = issue_content(&issue_body);
    let issue_number = env::var("ISSUE_NUMBER")?;

    if title.is_empty() || title.chars().any(|character| character.is_control()) {
        return Err("Issue title must be a single non-empty line".into());
    }
    if body.is_empty() {
        return Err("Issue body must not be empty".into());
    }

    let slug = slugify(&title);
    if slug.is_empty() {
        return Err("Issue title must contain at least one ASCII letter or number".into());
    }

    let timestamp = command_output("date", &["+%Y-%m-%d %H:%M:%S %z"])?;
    let date = timestamp
        .split_whitespace()
        .next()
        .ok_or("Missing publication date")?;
    let path = format!("_posts/{date}-{slug}.md");

    for entry in fs::read_dir("_posts")? {
        let candidate = entry?.path();
        if candidate
            .extension()
            .and_then(|extension| extension.to_str())
            != Some("md")
        {
            continue;
        }
        if source_issue_matches(&fs::read_to_string(&candidate)?, &issue_number) {
            let path = candidate.to_string_lossy().into_owned();
            let filename = candidate
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or("Published post has an invalid filename")?;
            let date = filename
                .get(..10)
                .ok_or("Published post has an invalid date")?;
            let slug = filename
                .get(11..)
                .ok_or("Published post has an invalid slug")?;
            write_outputs(&path, &date.replace('-', "/"), slug, false)?;
            println!("Issue #{issue_number} was already published as {path}");
            return Ok(());
        }
    }

    if Path::new(&path).exists() {
        return Err(format!("A post already exists at {path}; change the Issue title").into());
    }

    let yaml_title = title.replace('\'', "''");
    let tags = if explicit_tags.is_empty() {
        infer_tags(&title, body)
    } else {
        explicit_tags
    };
    let tags = tags
        .iter()
        .map(|tag| format!("'{tag}'"))
        .collect::<Vec<_>>()
        .join(", ");
    let front_matter = format!(
        "---\ntitle: '{yaml_title}'\ndate: {timestamp}\nshow_time: true\ncategory: Field notes\ntags: [{tags}]\ndescription: ''\nsource_issue: {issue_number}\n---\n"
    );

    let mut file = File::create(&path)?;
    write!(file, "{front_matter}\n{body}\n")?;

    write_outputs(&path, &date.replace('-', "/"), &slug, true)?;

    println!("Created {path} from Issue #{issue_number}");
    Ok(())
}

fn issue_content(issue_body: &str) -> (Vec<String>, &str) {
    let body = issue_body.trim();
    let (first_line, post) = body.split_once('\n').unwrap_or((body, ""));
    let Some(tags) = first_line.strip_prefix("Tags:") else {
        return (Vec::new(), body);
    };

    let mut explicit = Vec::new();
    for tag in tags.split(',').map(slugify) {
        if !tag.is_empty() && !explicit.contains(&tag) {
            explicit.push(tag);
        }
    }
    (explicit, post.trim())
}

fn infer_tags(title: &str, body: &str) -> Vec<String> {
    let mut tags = Vec::new();
    for text in [title, body] {
        let words = format!(" {} ", slugify(text).replace('-', " "));
        for (tag, aliases) in TAG_RULES {
            if tags.len() == 3 {
                return tags;
            }
            if !tags.iter().any(|selected| selected == tag)
                && aliases
                    .iter()
                    .any(|alias| words.contains(&format!(" {alias} ")))
            {
                tags.push((*tag).to_owned());
            }
        }
    }
    tags
}

fn source_issue_matches(post: &str, issue_number: &str) -> bool {
    let mut lines = post.lines();
    lines.next() == Some("---")
        && lines
            .take_while(|line| *line != "---")
            .filter_map(|line| line.strip_prefix("source_issue:"))
            .any(|value| value.trim() == issue_number)
}

fn write_outputs(path: &str, date_path: &str, slug: &str, created: bool) -> Result<()> {
    if let Ok(output) = env::var("GITHUB_OUTPUT") {
        let mut output_file = fs::OpenOptions::new().append(true).open(output)?;
        writeln!(output_file, "post_path={path}")?;
        writeln!(
            output_file,
            "post_url=https://rixrix.github.io/notes/{date_path}/{slug}/"
        )?;
        writeln!(output_file, "post_route=notes/{date_path}/{slug}/")?;
        writeln!(output_file, "created={created}")?;
    }
    Ok(())
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut needs_separator = false;

    for character in title.chars() {
        if character.is_ascii_alphanumeric() {
            if needs_separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character.to_ascii_lowercase());
            needs_separator = false;
        } else if !slug.is_empty() {
            needs_separator = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_up_to_three_tags_and_prefers_title_matches() {
        assert_eq!(
            infer_tags("Rust and GitHub Actions", "AWS, Azure, and Docker."),
            ["rust", "github-actions", "docker"]
        );
        assert!(infer_tags("Trust and sustainability", "There was no technical topic.").is_empty());
    }

    #[test]
    fn reads_optional_tags_without_changing_legacy_bodies() {
        assert_eq!(
            issue_content("Tags: Rust, Spec-driven development, Rust\r\n\r\nPost text."),
            (
                vec!["rust".into(), "spec-driven-development".into()],
                "Post text."
            )
        );
        assert_eq!(issue_content("Tags:\n\nPost text."), (vec![], "Post text."));
        assert_eq!(issue_content("Post text."), (vec![], "Post text."));
        assert_eq!(issue_content("Tags: Rust"), (vec!["rust".into()], ""));
    }

    #[test]
    fn matches_an_exact_issue_number_in_front_matter() {
        let post = "---\ntitle: Older post\nsource_issue: 10\n---\nsource_issue: 1\n";
        assert!(source_issue_matches(post, "10"));
        assert!(!source_issue_matches(post, "1"));
    }
}
