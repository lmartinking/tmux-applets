# tmux-applets

This is a work-in-progress conversion of the C code to Rust.

## Requirements

 * Linux 3.14 or above
 * Tmux v3.1 or above (24 bit colour support)
 * Terminal emulator which supports the above (basically everything except Terminal.app)

## Installation

```
cargo install tmux-applets
```

## Applets

Currently implemented:

 * `cpu`: Show CPU frequency usage
 * `mem`: Show memory usage
 * `ping`: Ping a host

## Parameters

To see available parameters, run: `tmux-applets --help`

## Usage in tmux

In `~/.tmux.conf`, edit your status line:

```
set-option -g status-right "#(/path/to/tmux-applets <applet> <arguments>) #(/path/to/tmux-applets <applet> <arguments>)"
```

For example, in my configuration I have:

```
set-option -g status-right "  CPU:#(/home/lucas/bin/tmux-applets cpu s:50 l:50)  MEM:#(/home/lucas/bin/tmux-applets mem pct-text s:50 l:50)  "
set-option -g status-right-length 48  # May be necessary if you have a long status line
set-option -g status-interval 1
```

## Contact

Bug reports, etc can be sent to <lmartinking@gmail.com>, or you can use
the github page at: <https://github.com/lmartinking/tmux-applets/issues>
