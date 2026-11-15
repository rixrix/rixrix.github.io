---
title: 'NodeJS Development Inside Docker on Windows 10'
date: 2017-02-15T16:39:23+13:00
category: Field notes
tags: [docker, linux, nodejs, windows-10]
description: ''
---

## Wait, what ?

Moving to Windows10 (6 months ago as of this writing) as my main OS is a bit hard, just a little bit.
I use commandline **ALL THE TIME** and I can't live without the developer's Swiss army knife on my side - tiny tools that make our life easier: _grep_, _sed_, _ps_, _tree_, _ssh_, _find_ ...

"_[Bash On Windows](https://msdn.microsoft.com/en-us/commandline/wsl/about)_" was a good strategy from Microsoft - I didn't bother about switching to Windows and I didn't think twice, I was more worried about which machine to buy.

I miss some of the good stuff from MacOS: [Homebrew](https://brew.sh/), [Homebrew Cask](https://caskroom.github.io/), to name a few.
I'll just get a Mac mini for OSX development.

## Frustration & Solution

I'm a member of the [Windows Insider Program](https://blogs.windows.com/blog/tag/windows-insider-program/) and their build releases includes fixes to some annoying issues, often with accompanying features.
However, the build release [15031](https://blogs.windows.com/windowsexperience/2017/02/08/announcing-windows-10-insider-preview-build-15031-pc/) affected my productivity.
Bash On Windows would hang up and opening up a separate Bash window gives the same result - you can't even close the window nor the running process - this would have been an easy `kill -9 <PID>` command in Mac/Linux, the famous ALT + CTRL + DELETE combination is nowhere effective.
NodeJS via NVM wouldn't work at all, see issue [#1683](https://github.com/Microsoft/BashOnWindows/issues/1683).

> Reboot! yep sometimes 6 reboots in a day

A rollback to the previous build release was the easiest solution. I hit the button and it works, as [Ben Hillis](https://github.com/Microsoft/BashOnWindows/issues/1683#issuecomment-279109940) said of Bash On Windows team:

> The rollback feature really is great and doesn't get enough credit. It's saved me a couple times as well.

The hang up went out however there are still minor issues like slowness, etc. It's bearable but I've already thought of finding an alternative,
one where I could easily jump from different machine and OS eg. MacOS, Linux.

## Docker, Docker

Enter Docker, I haven't fully utilised this tool since its beta phase. 5 or 6 montsh ago there were issues with mounting folders from host machine inside a container, it was a show stopper for me at that time.
I was following some steps from John Lees-Miller's "[Building a Node App in Docker](http://jdlm.info/articles/2016/03/06/lessons-building-node-app-docker.html)" and everthing works except for mounting a shared folder.

February 2017, I revisited steps and it works!

### Docker with Node Version Manager
Here's the slightly modified Docker configuration.

* OS: Ubuntu 16.04
* NodeJS: v6.9.2 via NVM
* NVM: v0.33.0
* Docker: 1.13.1 (10072)
* Docker Channel: Stable 94675c5

Dockerfile
```
FROM ubuntu:16.04

# Constants
ENV NVM_VERSION v0.33.0
ENV NODE_VERSION v6.9.2
ENV HOME=/home/app

# Replace shell with bash so we can source files
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# add special user called `app`
RUN useradd --user-group --create-home --shell /bin/false app

# Install pre-reqs
RUN apt-get update && apt-get install -y \
    build-essential \
    checkinstall    \
    libssl-dev      \
    curl

USER app

# Install NVM
RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/${NVM_VERSION}/install.sh | bash

# Install NODE
RUN source ~/.nvm/nvm.sh; \
    nvm install $NODE_VERSION; \
    nvm use --delete-prefix $NODE_VERSION;

USER root
COPY package.json start.sh $HOME/web/
RUN chown -R app:app $HOME/*

USER app
WORKDIR $HOME/web
RUN source ~/.nvm/nvm.sh; \
    nvm use --delete-prefix $NODE_VERSION; \
    npm install;

USER root
COPY . $HOME/web
RUN chown -R app:app $HOME/*

USER app
RUN source ~/.nvm/nvm.sh; \
    nvm use --delete-prefix $NODE_VERSION;

CMD ["./start.sh"]
```

docker-compose.yml

```
web:
  build: .
  ports:
    - '8080:8080'
  environment:
    NODE_ENV: dev
  volumes:
    - .:/home/app/web
    - /home/app/web/node_modules
```

start.sh

```
#!/bin/bash

# load NVM environment
source ~/.nvm/nvm.sh
nvm use --delete-prefix v6.9.2

# start server
npm start
```

Enjoy
