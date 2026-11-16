---
layout: default
title: About
description: Richard Sentino, a software engineer and lifelong tinkerer in Auckland, New Zealand.
manual_page: About(7)
permalink: /about/
phosphor_icons: true
---

# Richard Sentino

I’m a software engineer and lifelong tinkerer based in Auckland, New Zealand. I’ve spent more than 20 years working across startups, full-stack software, cloud platforms, and products that talk to hardware.

This is where I keep things I’ve tried and findings worth remembering. A strange bug, a side project, or something I learned from the person next to me all belong here. Lately, I’ve been writing about AI-assisted coding and spec-driven development, and getting back into Rust by using it in some of my side projects. My first Rust experiment was in 2015, when I tried getting it to talk to Node.js.

## What I’m building

{% assign traf = site.data.projects | where: 'name', 'TrafLang' | first %}

- <i class="ph ph-tree-structure" aria-hidden="true"></i> **AgenticFlowX** brings a spec-driven workflow for AI-assisted development into VS Code. [Project](https://agenticflowx.github.io/)
- <i class="ph ph-waveform" aria-hidden="true"></i> **Oxidal** is an open-source guitar and bass processor I’m building for Raspberry Pi 5. [Source](https://github.com/oxidalhq/oxidal)
- <i class="ph ph-sliders-horizontal" aria-hidden="true"></i> **Unofficial Desktop Nano Cortex** is a cross-platform desktop companion for the Neural DSP Nano Cortex. [Source](https://github.com/rixrix/deskop-nano-cortex)
- <i class="ph ph-code" aria-hidden="true"></i> **Traf** is a programming language I created with driving-themed commands like `StartEngine` and `Drive`. It compiles to JavaScript and runs in a browser-based editor. [Try Traf]({{ traf.demo_url }}) · [Source]({{ traf.url }})
{: .about-projects role="list" }

## Open-source contributions

- **TypeScript** · Added Web Audio’s `AudioContext.state` to the DOM typings. [Merged pull request](https://github.com/microsoft/TypeScript-DOM-lib-generator/pull/30)
- **Sodium TypeScript** · Worked on npm packaging and continuous integration. [Merged pull request](https://github.com/SodiumFRP/sodium-typescript/pull/27)
- **Firefox OS** · Submitted patches to Apache Cordova’s port for Mozilla’s Firefox OS, covering `deviceready` event handling and an error-message typo. [Event handling PR](https://github.com/apache/cordova-firefoxos/pull/1) · [Error message PR](https://github.com/apache/cordova-firefoxos/pull/5)
- **npm and Angular** · Filed a CLI documentation issue for npm and joined Angular discussions about routing and typings. [npm issue](https://github.com/npm/npm/issues/12223) · [Angular activity](https://github.com/search?q=is%3Apublic+involves%3Arixrix+org%3Aangular&type=issues)

## See also

- [Projects]({{ '/projects/' | relative_url }})
- [Notes archive]({{ '/notes/' | relative_url }})
- [GitHub](https://github.com/rixrix)
- [LinkedIn](https://www.linkedin.com/in/sentino/)
