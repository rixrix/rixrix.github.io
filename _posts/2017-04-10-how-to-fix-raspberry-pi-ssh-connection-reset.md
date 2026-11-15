---
title: 'How to fix Raspberry Pi SSH connection reset'
date: 2017-04-10T21:03:29+12:00
category: Field notes
tags: [raspberry-pi, raspbian, ssh]
description: ''
---

I was trying to copy files to my Raspbian-based Raspberry Pi but I kept getting this error:

> ssh: connect to host 192.168.178.79 port 22: Connection refused

## Tell-tale Sign

after few attempts the ssh status showed up an interesting set of logs:

$> sudo service ssh status
```
● ssh.service - OpenBSD Secure Shell server
   Loaded: loaded (/lib/systemd/system/ssh.service; enabled)
   Active: active (running) since Mon 2017-04-10 08:51:38 UTC; 6min ago
 Main PID: 1332 (sshd)
   CGroup: /system.slice/ssh.service
           └─1332 /usr/sbin/sshd -D

Apr 10 08:51:38 raspberrypi sshd[1332]: key_load_public: invalid format
Apr 10 08:51:38 raspberrypi sshd[1332]: Could not load host key: /etc/ssh/ss...y
Apr 10 08:51:38 raspberrypi sshd[1332]: key_load_public: invalid format
Apr 10 08:51:38 raspberrypi sshd[1332]: Could not load host key: /etc/ssh/ss...y
Apr 10 08:51:42 raspberrypi sshd[1338]: error: key_load_public: invalid format
Apr 10 08:51:42 raspberrypi sshd[1338]: error: Could not load host key: /etc...y
Apr 10 08:51:42 raspberrypi sshd[1338]: error: key_load_public: invalid format
Apr 10 08:51:42 raspberrypi sshd[1338]: error: Could not load host key: /etc...y
Apr 10 08:51:42 raspberrypi sshd[1338]: error: key_load_public: invalid format
Apr 10 08:51:42 raspberrypi sshd[1338]: error: Could not load host key: /etc...y
Hint: Some lines were ellipsized, use -l to show in full.
```

## Fix

googling brought me to this article [1], there are 2 suggested CLI commands but the command below fixed my issue:

`$> sudo rm /etc/ssh/ssh_host_* && sudo dpkg-reconfigure openssh-server`

It will generate a new set of ssh certs, like so:

```
Creating SSH2 RSA key; this may take some time ...
2048 73:a5:a5:1b:88:c6:19:40:0e:4c:51:18:ad:94:81:7f /etc/ssh/ssh_host_rsa_key.pub (RSA)
Creating SSH2 DSA key; this may take some time ...
1024 5a:37:ab:bf:11:c3:f5:ea:5f:e4:b3:ea:16:6a:11:f6 /etc/ssh/ssh_host_dsa_key.pub (DSA)
Creating SSH2 ECDSA key; this may take some time ...
256 d6:74:28:5e:ca:cd:ac:a8:69:32:03:f3:56:b5:48:7d /etc/ssh/ssh_host_ecdsa_key.pub (ECDSA)
Creating SSH2 ED25519 key; this may take some time ...
```
## My Raspberry Pi

* OS: Raspbian GNU/Linux 8.0 (jessie)
* Image burner [Etcher](+https://etcher.io/) tool
* [Raspberry Pi 1 Model B+ v1.2](https://www.raspberrypi.org/products/model-b-plus/)

## References

* [1] https://drjohnstechtalk.com/blog/2015/04/cant-ssh-to-raspberry-pi/
* [2] https://etcher.io/
* [3] https://www.raspberrypi.org/products/model-b-plus/
