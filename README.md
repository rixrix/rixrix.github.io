# Richard Sentino · Software Notes

[Read the blog at rixrix.github.io](https://rixrix.github.io/)

Field notes on software development, side projects, and experiments with Rust and AI-assisted coding. The homepage shows the latest post, and the [Notes archive](https://rixrix.github.io/notes/) lists all posts.

## Writing and publishing

[`_blog.rs`](_blog.rs) is a single-file helper for creating, building, and publishing Markdown posts. It runs with [rust-script](https://rust-script.org/). You’ll also need Git and Jekyll. The script uses the local Jekyll installation in `.jekyll-gems` when available and otherwise looks for `jekyll` on your PATH.

Run these commands from the repository root:

```sh
rust-script _blog.rs new "My next post"
rust-script _blog.rs build
rust-script _blog.rs publish _posts/YYYY-MM-DD-my-next-post.md
```

1. **New** creates a dated Markdown file in `_posts` and prints its path. Open it and write your post.
2. **Build** renders the site locally into `_site`. This step is optional because publishing builds the site too.
3. **Publish** takes the path printed by `new`. From `main`, it builds the site, commits the selected post, and pushes to GitHub. GitHub Pages deploys the update automatically.

Run `rust-script _blog.rs --help` to see the commands again.
