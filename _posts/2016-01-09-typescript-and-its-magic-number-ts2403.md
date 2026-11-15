---
title: 'TypeScript and its magic number TS2403'
date: 2016-01-09T10:59:13+12:00
category: Field notes
tags: [nodejs, typescript]
description: ''
---

At work, we’ve been using Webpack for our frontend stack (TypeScript, AngularJS v1.x, NodeJS, GulpJS + other JS libraries without much drama.
Over the last couple of months our Angular2-based project is starting to get a life, from a simple PoC to a usable web app.
Though we were working against the early alpha releases of Angular2 but we thought it’s gonna be fine — we just want a minimum viable product.
Subsequent bumps to Angular2 release builds up to the first beta the error message below started to popped out and now our simple Webpack `require` interface needs some little attention.

```
…node_modules/angular2/typings/node/node.d.ts
error TS2403: Subsequent variable declarations must have the same type.
Variable ‘require’ must be of type ‘NodeRequire’, but here has type ‘WebpackRequire’.
```

ah easy fix!, I thought it was just a 2 minutes fix with TypeScript’s “intersection types” introduced in 1.6 release:

```
declare var require: NodeRequire | WebpackRequire
```

well that didn’t work!

```
…node_modules/angular2/typings/node/node.d.ts
error TS2403: Subsequent variable declarations must have the same type.
Variable ‘require’ must be of type ‘NodeRequire’, but here has type ‘NodeRequire | WebpackRequire’.
```

Fiddling around the internets brought me to a Github issue https://github.com/Microsoft/TypeScript/issues/3215.
One of the commenter pointed out to make use of TypeScript open-ended interface functionality which sort of acts like overriding an existing type definition.
It’s supposedly pretty straight forward to just modify the offending type definition file by changing it to `declare var require: any` however it’s a file shipped via npm package,
the quick fix might work today but we don’t know after your next `npm install` your fix will still be there.

#### Anyway here’s the updated interface:

```
interface WebpackRequire {
    <T>(path: string): T;
    (paths: string[], callback: (...modules: any[]) => void): void;
    ensure: (paths: string[], callback: (require: <T>(path: string) => T) => void) => void;
}
interface NodeRequire extends WebpackRequire {}
declare var require: NodeRequire;
```

I tried to make the updated interface definition to be little bit verbose just to remind that this is an overridden (as I call it) interface specifically for Webpack.

#### References

* https://github.com/Microsoft/TypeScript/issues/3215#issuecomment-103467086
* https://basarat.gitbooks.io/typescript/content/docs/types/ambient/interfaces.htm
* http://ronniehegelund.blogspot.co.nz/2013/01/extension-methods-in-typescript.html
* https://github.com/angular/angular/issues/5807#issuecomment-164678165
* https://github.com/angular/angular/issues/6266
