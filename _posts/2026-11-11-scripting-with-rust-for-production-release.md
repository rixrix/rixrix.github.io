---
title: Scripting with Rust for production release
date: 2026-11-11 00:10:00 +1300
category: Field notes
tags: [rust, deployment, rust-script]
description: A small experiment with a single-file Rust deployment script.
---

At work, my team has room to experiment as long as the impact is small if something goes wrong. One of those experiments was a simple `deploy.rs` for our marketing website. It deploys to DEV and PROD, and it works.

I used [rust-script](https://rust-script.org/) so I could keep the script and its dependencies in one file. The shebang tells the shell what to run, and the fenced `cargo` block declares the dependencies.

Here’s the shape of it, using the [timestamp example from the rust-script docs](https://rust-script.org/#scripts):

````rust
#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! time = "0.1.25"
//! ```
fn main() {
    println!("{}", time::now().rfc822z());
}
````

I’d mostly thought of Rust as something I’d reach for when building an application. It was nice to reach for it for one small release script, too.
