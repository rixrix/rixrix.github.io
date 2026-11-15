---
title: 'That! SSH permission denied on eCryptfs-enabled file system'
date: 2017-06-22T23:00:25+12:00
category: Field notes
tags: [hacks, ssh]
description: ''
---

On the day my [DeskMini](/notes/2017/05/11/asrock-h110m-stx-deskmini-110w-with-headless-ubuntu-server/) Headless server was born -
it was christened with [ecryptfs](http://ecryptfs.org/) and I did that on purpose - paranoid!

Gearing up for a password-less configuration was supposedly and easy task later then I realised that my first SSH session won't work.
This is for obvious reason: the home folder `~/` isn't readable - yet - not until you manually mount the folder.
By default SSH will look for `authorized_keys` file at the user's home folder henced SSH can't read it - you'll get `Permission denied (publickey).`

One way to fix it, as suggested by others is to configure ssh and set the `AuthorizedKeysFile` to a different readable path

sshd_config:
```
AuthorizedKeysFile /etc/ssh/keys/authorized_keys
```

If you allow multiple SSH users on your machine, at some stage they'll get denied - unmounted user's home folder etc.
Use the same fix above but use `%u` user TOKEN. This will let the SSH server to open up authorized_keys file based on the current user's _username_ trying to connect via SSH.

sshd_config:
```
AuthorizedKeysFile /etc/ssh/keys/%u/authorized_keys
```

Example:

```
$> ssh mrtrump@192.168.1.99
```

SSH server will look for authorized_keys file at `/etc/ssh/keys/mrtrump/authorized_keys`

Enjoy!

## References
* https://man.openbsd.org/sshd_config#TOKENS
