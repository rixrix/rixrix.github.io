---
title: 'Parceljs in Production! (almost)'
date: 2018-11-24T19:08:14+13:00
category: Field notes
tags: [notes]
description: ''
---

See this [Parcel](https://parceljs.org/) slogan:

> Blazing fast, zero configuration web application bundler

Prior to using it, I saw a number of articles about it and 
I was like "nice!, finally a tool that allows me to focus on building stuff instead of fine tunning the build infrastructure"

Few months after I got the task to do an integration work.
This involves building a custom plugin from a 3rd party system.
We get a mixed of raw .js and a transpiled .js

```
|_ dist/
|_ public/
  |_ index.html
|_ vendor/
  |_ vendor.min.js
|_ src/
  |_ custom.js
  |_ custom.sass
  |_ some.css
```

**The First 30mins**

Our project was initially built on top of [Babel](https://babeljs.io/) however we needed
to do more than Babel does eg. SASS, CSS, auto-reload. 
I decided to give a go with Parcel, hooking it up was less of a work - Zero Configuration, fast build time, auto-reload.

**The Problem**

Everything works! the fact that I could skip out Webpack completely was a huge win.
However I noticed my `vendor.min.js` gets double-transpiled and this has caused my whole
dev day finding a better solution or a temporary work-around. There were suggestions and recommendations but I was not totally convinced.

There's no (better) way for me to mark or skip `vendor.min.js` from Parcel's build workflow.

**Production (almost)**

I pushed back and decided to go with the simplified version: 

- Babel
- Nodemon
- Node-SASS

**Optional Configuration**

Zero Configuration is an ambitious attempt, definitely I was 95-98% complete - so they're getting there, it will work with other scenario. 
An optional configuration might work in this case. 

Turns out quite a few people are having this issue. There's an on-going ticket about this:

> **How do i mark some modules to external?** https://github.com/parcel-bundler/parcel/issues/144

Please support Parcel project. It's an awesome tool and I'm looking forward to using it in some other project.

**Further reading**

- The Problem with Webpack and Why It Is (Kind of) Our Fault https://medium.com/@allanbaptista/the-problem-with-webpack-8a025268a761
- If you’ve ever configured Webpack, Parcel will blow your mind! https://medium.com/@ibrahimbutt/if-youve-ever-configured-webpack-parcel-will-blow-your-mind-b615468cee78
- Comparing bundlers: Webpack, Rollup & Parcel https://medium.com/js-imaginea/comparing-bundlers-webpack-rollup-parcel-f8f5dc609cfd
