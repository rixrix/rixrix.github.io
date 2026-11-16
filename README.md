# Welcome!

[Read the blog at rixrix.github.io](https://rixrix.github.io/)

Field notes on software development, side projects, and experiments with Rust and AI-assisted coding. The homepage links to the latest posts and selected projects. [Notes archive](https://rixrix.github.io/notes/) · [RSS feed](https://rixrix.github.io/feed.xml)

## Writing and publishing

[`_blog.rs`](_blog.rs) is a single-file helper for creating, building, and publishing Markdown posts. Use the Ruby version in `.ruby-version` and run `bundle install` to install the locked Jekyll dependencies. You’ll also need Git and [rust-script](https://rust-script.org/).

Run these commands from the repository root:

```sh
rust-script _blog.rs new "My next post"
rust-script _blog.rs build
rust-script _blog.rs publish _posts/YYYY-MM-DD-my-next-post.md
```

1. **New** creates a dated Markdown file in `_posts` and prints its path. Open it and write your post.
2. **Build** renders the site locally into `_site` and checks internal links, assets, anchors, and the RSS feed. Publishing also runs this check.
3. **Publish** takes the path printed by `new`. From `main`, it builds the site, commits the selected post, and pushes to GitHub. GitHub Pages deploys the update automatically.

Run `rust-script _blog.rs --help` to see the commands again.

## Publishing from a GitHub Issue

Open an Issue using the **Blog post** template. On the first line, leave `Tags:` empty for automatic tags or enter a comma-separated list. Leave a blank line before the post body. The publisher uses up to three matches from the title and body when the field is empty. Existing issues that start with the post body are still supported.

Add the `publish` label when the post is ready. Actions checks the generated site, commits the new post, deploys it, and marks the Issue published after the live URL matches the build. Rerunning a publish fetches the existing post and refreshes its publication status.

## GitHub Pages

With **GitHub Actions** selected as the Pages source, the **Build and deploy blog** workflow runs on pushes to `main`, Issue publishes, or a manual run from `main`. Pull requests build and validate the site. For the sidebar’s **From the archive** link, edit `archive_pick` in `_config.yml`.
