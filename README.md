# tmux-applets

This is a work-in-progress conversion of the C code to Rust.

## Installation

```
cargo install tmux-applets
```

## Applets

Currently implemented:

 * `cpu`: Show CPU frequency usage

TODO:

 * `mem`: Show memory usage
 * `ping`: Ping a host

## Usage in tmux

In `~/.tmux.conf`, edit your status line:

```
set-option -ag status-right "#(/path/to/applet <arguments>) #(/path/to/applet2 <arguments>)"
```

For example, in my configuration I have:

```
set-option -ag status-right " CPU: #(/home/lucas/.cargo/bin/tmux-applets cpu s:50 l:50)"
set-option -ag status-interval 1
```

## Contact

Bug reports, etc can be sent to <lmartinking@gmail.com>, or you can use
the github page at: <https://github.com/lmartinking/tmux-applets/issues>
