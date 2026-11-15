---
title: 'Install FFmpeg from source on Ubuntu Bash on Win10'
date: 2016-11-28T01:17:35+13:00
category: Field notes
tags: [bash, ubuntu, windows-10]
description: ''
---

I was working on transcoding videos into different formats, resolutions so it could be played on various desktops, tablets and mobile devices.
My main working machine is Dell XPS13 on Windows 10 with Ubuntu Bash enabled, some of my needed libraries is missing and needs recompilation of FFmpeg the one
that came from Ubuntu 14.04 repo.

I think I've literraly spent hours or probably days buildin/re-building libraries just to get the videos transcoded into my desired format.

I got tired of copy/pasting the script from the [wiki](https://trac.ffmpeg.org/wiki/CompilationGuide/Ubuntu). I also needed a script where I could drop into my build server.
There are couple of examples online, I've tried some of those, and some are a bit outdated now.

I ripped off the FFmpeg compilation steps and dropped into a simple bash script. All I needed to do now is just:

* `sudo installer.sh`
* `sudo uninstaler.sh`

grab the script here: https://github.com/rixrix/ffmpeg_ubuntu_installer.sh
