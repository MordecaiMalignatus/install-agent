# `install-agent` -- Because god, plists are terrible.

You work from a macbook, either work's or your own. You have a program you want
to run on a schedule. You reach for cron. You realise cron skips runs when the
laptop happens to be sleeping at the time, is shut off, or any million other
things. You do some reading. You reach for `launchd`, the macOS init system. You
realise the input format are terribly janky XML files that are element-order
dependent. You build a VPS on Digital Ocean and run cron there, instead.

But wait! There's `install-agent`, something that's a bit like `crontab -e`, but
for `launchd`! You stop having to look up syntax and locations every time, and
actually use the init system like intended.

## Usage

### Install a new agent

```sh
$ install-agent awesome-agent --command '/usr/bin/awesome-command --frobnicate' --interval 10m
```

This creates a new agent labelled `awesome-agent`, running the command
`/usr/bin/awesome-command --frobnicate` every 10 minutes.

### List agents handled by `install-agent`

```sh
$ install-agent list
```

All agents handled by `install-agent` get a little extra marking that makes it
easy to identify and handle the agents installed/edited with `install-agent`.
For all jobs, regardless if installed/edited by `install-agent`, add the `--all`
flag.
