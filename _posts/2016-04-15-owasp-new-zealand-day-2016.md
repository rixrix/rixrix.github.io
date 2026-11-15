---
title: 'OWASP New Zealand Day 2016'
date: 2016-04-15T16:18:14+13:00
category: Field notes
tags: [conference, owasp, security]
description: ''
---

This is kind of an old story now, 1+ month ago as of this writing.

---

The conference had a good [list of speakers](https://www.owasp.org/index.php/OWASP_New_Zealand_Day_2016#tab=Presentation_Schedule) and most of them are working for a NZ-based security companies.
There were shown pre-recorded demos and live ones as well.

One of the highlights would be the lessons learned from this conference:

* **Credit Card Fraud ([pdf](https://web.fredden.org/assets/OWASP%20Feb%202016.pdf))** — by Dan Wallis of Christchurch ISIG
* **Content Security Policy ([pdf](https://www.owasp.org/images/9/9e/Keep_Calm_And_CSP.pdf))** — by Valentinas Bakaitis of Aura Security
* **Oauth 2.0: The Promise and Pitfalls ([pdf](https://www.lateralsecurity.com/downloads/Lateral_Security-OAuth2_The_Promise_and_Pitfalls_NZOWASP2016.pdf))** — by Sergey Ozernikov of Lateral Security
* **Don’t be a Hero** — There was one talk where the speaker did not recommend on rolling your own crypto, or re-inventing the wheel — it’s hard — there are a dime dozen open source and battle tested crypto tools floating around. *I totally forgot the speaker and topic title, my apology*.
* **Same Origin Policy** — When you set the following configuration `Access-Control-Allow-Origin: *`, you need to seriously consider the implications of such one-line hallelujah. There was a short demo of hijacking yahoo mail cookies and taking over user’s account with a simple Java applet.
* **Continuous Security** — by Laura Bell of SafeStack. The topic was about making security be part of the development process and not an afterthought — it’s where you only thought of security after the project is done and deployed to production. Another recommendation was to automate everything as possible to reduce the possibility human mistake… heard of DevOps ?
* **Deserialization, what could go wrong** — by Brendan Jamieson of Insomnia Security. It mainly talks about the potential danger of deserialising user controlled data. The presenter showed a few hands on demo in PHP, Python and Java — awesome!
* **Using 3rd Party Framework** — Pick out the one with good and active community and are always up to date. New found/reported vulnerabilities would be patched in days and not weeks or months
* **Security Knowledge Framework (SKF) ([link](https://www.securityknowledgeframework.org/))** — from their website “SKF is an open source security knowledgebase including manageable projects with checklists and best practice code examples in multiple programming languages showing you how to prevent hackers gaining access and running exploits on your application.”. I highly recommend that you check out their website at https://www.securityknowledgeframework.org/

---

Things that didn’t go well for me, pretty minor

Room capacity issue — There was 1 talk where a bunch of people waiting outside but were not allowed to get inside due to capacity issue.
I thought, well, we’re techy people and we don’t mind standing and sitting on the floor.
Most of the people outside — including me — just wanted the technical knowledge, the tips.
A bit disappointing but nonetheless it was good.

---

A huge thanks to the OWASP NZ organizers, sponsors and the speakers.
It was worth it, good location, good crowd!
