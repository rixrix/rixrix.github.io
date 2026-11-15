---
title: 'Go1.7 installation via GVM on Ubuntu Bash on Windows 10'
date: 2016-11-26T22:27:10+13:00
category: Field notes
tags: [bash, tools, windows-10]
description: ''
---

Since moving to [Hugo](https://www.gohugo.io/) few months ago my installation
history with [Go](https://golang.org/) was quick and dirty, I just want to get something
up running to get started with Hugo. 48 hours ago I was playing with Go,
I thought of building a simple cli utility where CI server can hook in and start building something
- of course it has be built on top of Go, enough to get my around its dev-ecosystem.

Bash on Windows 10 shipped with Ubuntu 14.04.5 but unfortunately the repo contains Go1.6.
I could use the Personal Package Archive (ppa) repo however coming from Node Version Manager (nvm),
I find it a lot easier to switchover from one version to another -
I have NodeJS projects which requires specific version to run eg: 0.x, 4.x and 6.x and so a version manager is a big help.

---

#### Installation
it's pretty straight forward, see gvm steps at https://github.com/moovweb/gvm#installing

#### Notes
These are just a few details that I've encountered during gvm installation, it might be of your help.

#### Failed to install Go1.7

```bash
$> gvm install go1.7

Downloading Go source...
Installing go1.7...
 * Compiling...
ERROR: Failed to compile. Check the logs at /home/urix/.gvm/logs/go-go1.7-compile.log
ERROR: Failed to use installed version
```

Make sure to set to issue this command before you install v1.7

```bash
gvm install go1.4 -B
gvm use go1.4
export GOROOT_BOOTSTRAP=$GOROOT
gvm install go1.7
```

Set the default Go1.7 version to get the same version when you close and open the Bash shell

```bash
gvm use go1.7 --default
```

#### GOROOT and GOPATH
These environment variables will be set automatically by gvm for you when you switch versions,
which is handy when testing packages with different versions of Go.

My environment for Go1.7

```bash
$> go version
go version go1.7 linux/amd64

$> go env

GOARCH="amd64"
GOBIN=""
GOEXE=""
GOHOSTARCH="amd64"
GOHOSTOS="linux"
GOOS="linux"
GOPATH="/home/urix/.gvm/pkgsets/go1.7/global"
GORACE=""
GOROOT="/home/urix/.gvm/gos/go1.7"
GOTOOLDIR="/home/urix/.gvm/gos/go1.7/pkg/tool/linux_amd64"
CC="gcc"
GOGCCFLAGS="-fPIC -m64 -pthread -fmessage-length=0 -fdebug-prefix-map=/tmp/go-build356629057=/tmp/go-build -gno-record-gcc-switches"
CXX="g++"
CGO_ENABLED="1"

```

and Go1.4

```bash
$> gvm use go1.4
Now using version go1.4

$> go version
go version go1.4 linux/amd64

$> go env
GOARCH="amd64"
GOBIN=""
GOCHAR="6"
GOEXE=""
GOHOSTARCH="amd64"
GOHOSTOS="linux"
GOOS="linux"
GOPATH="/home/urix/.gvm/pkgsets/go1.4/global"
GORACE=""
GOROOT="/home/urix/.gvm/gos/go1.4"
GOTOOLDIR="/home/urix/.gvm/gos/go1.4/pkg/tool/linux_amd64"
CC="gcc"
GOGCCFLAGS="-fPIC -m64 -pthread -fmessage-length=0"
CXX="g++"
CGO_ENABLED="1"
```
