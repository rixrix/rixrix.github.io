---
layout: default
title: About
description: Richard Sentino, a software engineer and lifelong tinkerer in Auckland, New Zealand.
manual_page: About(7)
permalink: /about/
phosphor_icons: true
---

# Richard Sentino

I’m a software engineer and lifelong tinkerer based in Auckland, New Zealand. I’ve spent more than 20 years building across startups, full-stack software, cloud platforms, and products that talk to hardware.

This site is where I keep the things worth remembering: a strange bug, a side project, an open-source contribution, or something I learned from the person next to me. Lately I’ve been writing about AI-assisted coding, spec-driven development, and getting back into Rust through side projects. My first Rust experiment was in 2015, when I tried getting it to talk to Node.js.

## What I’m building

{% assign traf = site.data.projects | where: 'name', 'TrafLang' | first %}

- <i class="ph ph-tree-structure" aria-hidden="true"></i> **AgenticFlowX** brings a spec-driven workflow for AI-assisted development into VS Code. [Project](https://agenticflowx.github.io/)
- <i class="ph ph-waveform" aria-hidden="true"></i> **Oxidal** is an open-source guitar and bass processor I’m building for Raspberry Pi 5. [Source](https://github.com/oxidalhq/oxidal)
- <i class="ph ph-sliders-horizontal" aria-hidden="true"></i> **Unofficial Desktop Nano Cortex** is a cross-platform desktop companion for the Neural DSP Nano Cortex. [Source](https://github.com/rixrix/deskop-nano-cortex)
- <i class="ph ph-code" aria-hidden="true"></i> **Traf** is a programming language I created with driving-themed commands like `StartEngine` and `Drive`. It compiles to JavaScript and runs in a browser-based editor. [Try Traf]({{ traf.demo_url }}) · [Source]({{ traf.url }})
{: .about-projects role="list" }

## Open source, over time

Small contributions count. These are a few that survived the years:

- **m0n0dev / m0n0wall** · Submitted a patch to m0n0dev, a tool for building m0n0wall-based distributions. It was committed to the main trunk in 2008. [Original post]({{ '/notes/2008/02/26/m0n0dev-patch/' | relative_url }}) · [Archived repository](https://github.com/rixrix/m0n0dev)
- **Angular 2** · Made a small contribution during the pre-release period. `rixrix` appears among the top contributors to 2.0.0-alpha.37, and Richard Sentino is named in the final release acknowledgement. [The story and screenshots]({{ '/notes/2026/11/21/a-small-footnote-in-angular-2/' | relative_url }})
- **TypeScript** · Added Web Audio’s `AudioContext.state` to the DOM typings. [Merged pull request](https://github.com/microsoft/TypeScript-DOM-lib-generator/pull/30)
- **Sodium TypeScript** · Worked on npm packaging and continuous integration. [Merged pull request](https://github.com/SodiumFRP/sodium-typescript/pull/27)
- **Firefox OS** · Submitted patches to Apache Cordova’s port for Mozilla’s Firefox OS, covering `deviceready` event handling and an error-message typo. [Event handling PR](https://github.com/apache/cordova-firefoxos/pull/1) · [Error message PR](https://github.com/apache/cordova-firefoxos/pull/5)
- **npm** · Filed a CLI documentation issue. [Issue](https://github.com/npm/npm/issues/12223)

## See also

- [Projects]({{ '/projects/' | relative_url }})
- [Notes archive]({{ '/notes/' | relative_url }})
- [GitHub](https://github.com/rixrix)
- [LinkedIn](https://www.linkedin.com/in/sentino/)
