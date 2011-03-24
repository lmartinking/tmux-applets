CFLAGS=-Os
CC=gcc
STRIP=strip

INSTALL_DIR=/home/$(USER)/bin

.PHONY: clean

tmux-cpu-freq: tmux-cpu-freq.c
	$(CC) $(CFLAGS) -o tmux-cpu-freq tmux-cpu-freq.c
	$(STRIP) -s tmux-cpu-freq

clean:
	rm -f tmux-cpu-freq	

install: tmux-cpu-freq
	install -D tmux-cpu-freq $(INSTALL_DIR)/tmux-cpu-freq
