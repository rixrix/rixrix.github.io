---
title: 'NodeJS + Cron: A series of attempts in getting rid of dangling processes'
date: 2017-02-20T20:47:01+13:00
category: Field notes
tags: [cron, nodejs]
description: ''
---

Short-lived scripts that sucked your resources for 1mins or at most 5mins - is probably fine.
However, if it is recurring jobs in Cron and involves several tasks and at least an hour of processing - that'd be a different story.

I was working on a simple NodeJS-based cron script with only 3 requirements:

* extract
* transform
* load

Each one of these however would involve crunching through a million rows of data.

If you take a closer look at the result of `ps aux` command, you'll notice that there are couple of dangling processes waiting for some event to complete - of which technically the parent process doesn't exists anymore! - the dates would tell you that some of those process had been there for about 9 days.

the script runs on top of AWS EC2 instance so literally we're talking about:

> CPU ticks, RAM usage, disk spins, etc == **$$$**

If left unattended this would turn into a zombie manufacturing plant. Anyhow, see the command output below.

`$> ps aux`:
```
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root      6934  0.0  0.0  59636   448 ?        S    Feb10   0:00 CRON
ubuntu    7005  0.0  0.0   4440   104 ?        Ss   Feb10   0:00 /bin/sh -c /home/ubuntu/crontools/cron-script-wrapper.sh > /dev/null 2>&1
ubuntu    7008  0.0  0.0  11112   180 ?        S    Feb10   0:00 /bin/bash /home/ubuntu/crontools/cron-script-wrapper.sh
ubuntu    7094  0.0  0.0   4444   108 ?        S    Feb10   0:00 sh -c npm run export-task && npm run transform-task && npm run load-task
ubuntu    7095  0.0  0.1 574624  5924 ?        Sl   Feb10   0:00 npm run export-task
ubuntu   15536  0.0  0.0   4440   104 ?        Ss   Feb18   0:00 /bin/sh -c /home/ubuntu/crontools/cron-script-wrapper.sh > /dev/null 2>&1
ubuntu   15557  0.0  0.0  11112   176 ?        S    Feb18   0:00 /bin/bash /home/ubuntu/crontools/cron-script-wrapper.sh
ubuntu   19398  0.0  0.0   4440   100 ?        Ss   Feb11   0:00 /bin/sh -c /home/ubuntu/crontools/cron-script-wrapper.sh > /dev/null 2>&1
ubuntu   19399  0.0  0.0  11112   192 ?        S    Feb11   0:00 /bin/bash /home/ubuntu/crontools/cron-script-wrapper.sh
ubuntu   19410  0.0  0.0   4444   108 ?        S    Feb11   0:00 sh -c npm run export-task && npm run transform-task && npm run load-task
ubuntu   19421  0.0  0.0   4444   108 ?        S    Feb11   0:00 sh -c node ./js/cron-script.js export-task
ubuntu   19422  0.0  1.1 839088 44164 ?        Sl   Feb11   0:01 node ./js/cron-script.js export-task
ubuntu   25683  0.0  0.0   4440   100 ?        Ss   Feb14   0:00 /bin/sh -c /home/ubuntu/crontools/cron-script-wrapper.sh > /dev/null 2>&1
ubuntu   25684  0.0  0.0  11112   188 ?        S    Feb14   0:00 /bin/bash /home/ubuntu/crontools/cron-script-wrapper.sh
ubuntu   25712  0.0  0.0   4444   104 ?        S    Feb14   0:00 sh -c npm run export-task && npm run transform-task && npm run load-task
ubuntu   25723  0.0  0.0   4444   104 ?        S    Feb14   0:00 sh -c node ./js/cron-script.js export-task
ubuntu   25724  0.0  2.6 964268 102912 ?       Sl   Feb14   0:02 node ./js/cron-script.js export-task
ubuntu   26093  0.0  0.0   4440   100 ?        Ss   Feb14   0:00 /bin/sh -c /home/ubuntu/crontools/cron-script-wrapper.sh > /dev/null 2>&1
ubuntu   26094  0.0  0.0  11112   188 ?        S    Feb14   0:00 /bin/bash /home/ubuntu/crontools/cron-script-wrapper.sh
ubuntu   26107  0.0  0.0   4444   108 ?        S    Feb14   0:00 sh -c npm run export-task && npm run transform-task && npm run load-task
ubuntu   26118  0.0  0.0   4444   108 ?        S    Feb14   0:00 sh -c node ./js/cron-script.js export-task
ubuntu   26119  0.0  1.5 929424 60320 ?        Sl   Feb14   0:01 node ./js/cron-script.js export-task
ubuntu   26511  0.0  0.0   4440   104 ?        Ss   Feb09   0:00 /bin/sh -c /home/ubuntu/crontools/cron-script-wrapper.sh > /dev/null 2>&1
ubuntu   26512  0.0  0.0  11112   188 ?        S    Feb09   0:00 /bin/bash /home/ubuntu/crontools/cron-script-wrapper.sh
ubuntu   26523  0.0  0.0   4444   108 ?        S    Feb09   0:00 sh -c npm run export-task && npm run transform-task && npm run load-task
ubuntu   26534  0.0  0.0   4444   104 ?        S    Feb09   0:00 sh -c node ./js/cron-script.js export-task
ubuntu   26535  0.0  0.7 601220 28212 ?        Sl   Feb09   0:01 node ./js/cron-script.js export-task
```

On the day when the script was loaded onto crontab and didn't get was I was hoping for, my gut tells me that I f***ed up Cron: bad syntax or wasn't throwing the logs properly, why? you asked! - having passed the "[Works on My Machine](https://blog.codinghorror.com/the-works-on-my-machine-certification-program/)" certification program I could run the script manually at sucess rate of 100%.

> ¯\\\_(ツ)_/¯
>
> E-V-E-R-Y-T-H-I-N-G WORKS!

### Unsuccessful Attempts

#### 1st attempt:

```
* * * * * * cd /to/script/folder && npm run export-task && npm run transform-task && npm run load-task 2>&1
```

#### 2nd attempt:

```
* * * * * * cd /to/script/folder && npm run-task-in-sequence 2>&1
```

#### 3rd attempt:

```
* * * * * * cd /to/script/folder && ./script-wrapper.sh 2>&1
```

script-wrapper.sh
```
#!/bin/bash

npm run export-task
npm run transform-task
npm run load-task

exit 0
```

### Successful Attempts

#### 4th attempt:

```
* * * * * * cd /to/script/folder && ./script-wrapper.sh 2>&1
```

script-wrapper.sh
```
#!/bin/bash

node ./cron-script.js export-task
node ./cron-script.js transform-task
node ./cron-script.js load-task

exit 0
```

Not relying on running [arbitrary commands](https://docs.npmjs.com/cli/run-script) in npm did fixed the issue.

### Notes

* The issue did went away on the 4th attempt but it still needs a lot of explaining because the main issue occurs only inside AWS/EC2 machine. I couldn't replicate the issue on a Docker machine
* I forgot to get the snapshot of the real zombie process, see [1]


### References

* [1] https://www.howtogeek.com/119815/htg-explains-what-is-a-zombie-process-on-linux/
* [2] https://docs.npmjs.com/cli/run-script
* [3] https://blog.codinghorror.com/the-works-on-my-machine-certification-program/
