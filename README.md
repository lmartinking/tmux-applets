# tmux-applets

This is work-in-progress.

## Installation

```
cargo install tmux-applets
```

## Usage in tmux
	
In ~/.tmux.conf, edit your status line:

```
set status-right "#(/path/to/applet <arguments>) #(/path/to/applet2 <arguments>)"
```

For example, in my configuration I have:

```
set status-right "#(/home/lucas/.cargo/bin/tmux-applets cpu-freq)  "
set status-interval 1
```

## Contact

Bug reports, etc can be sent to <lmartinking@gmail.com>, or you can use
the github page at: <https://github.com/lmartinking/tmux-applets/issues>
