use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let title = env::var("ISSUE_TITLE")?.trim().to_owned();
    let body = env::var("ISSUE_BODY")?.trim().to_owned();
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

    let date = command_output("date", &["+%Y-%m-%d"])?;
    let timestamp = command_output("date", &["+%Y-%m-%d %H:%M:%S %z"])?;
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
        if fs::read_to_string(&candidate)?.contains(&format!("source_issue: {issue_number}")) {
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
    let front_matter = format!(
        "---\ntitle: '{yaml_title}'\ndate: {timestamp}\ncategory: Field notes\ntags: []\ndescription: ''\nsource_issue: {issue_number}\n---\n"
    );

    let mut file = File::create(&path)?;
    write!(file, "{front_matter}\n{body}\n")?;

    write_outputs(&path, &date.replace('-', "/"), &slug, true)?;

    println!("Created {path} from Issue #{issue_number}");
    Ok(())
}

fn write_outputs(path: &str, date_path: &str, slug: &str, created: bool) -> Result<()> {
    if let Ok(output) = env::var("GITHUB_OUTPUT") {
        let mut output_file = fs::OpenOptions::new().append(true).open(output)?;
        writeln!(output_file, "post_path={path}")?;
        writeln!(
            output_file,
            "post_url=https://rixrix.github.io/notes/{date_path}/{slug}/"
        )?;
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
