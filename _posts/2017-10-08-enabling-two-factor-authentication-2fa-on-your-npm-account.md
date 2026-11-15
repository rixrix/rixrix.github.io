---
title: 'Enabling Two-Factor Authentication (2FA) on your npm account'
date: 2017-10-08T00:28:54+13:00
category: Field notes
tags: [npm, today-i-learned]
description: ''
---

4 days ago, npm published a [blog](http://blog.npmjs.org/post/166039777883/protect-your-npm-account-with-two-factor) about protecting your account with 2FA. I'm a user of Google Authenticator [[1]](https://play.google.com/store/apps/details?id=com.google.android.apps.authenticator2&hl=en), [[2]](https://itunes.apple.com/nz/app/google-authenticator/id388497605?mt=8) and [YubiKey](https://www.yubico.com/) - so it was just perfect timing.

To enable 2FA on your profile, you need to install the beta release of npm client:

```
$> npm install npm@next -g
```

and follow the rest of the instruction here: https://docs.npmjs.com/getting-started/using-two-factor-authentication
