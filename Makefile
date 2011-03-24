CFLAGS=-Os
CC=gcc
STRIP=strip

INSTALL_DIR=/home/$(USER)/bin

.PHONY: clean all

all: tmux-cpu-freq tmux-ping

tmux-cpu-freq: tmux-cpu-freq.c
	$(CC) $(CFLAGS) -o tmux-cpu-freq tmux-cpu-freq.c
	$(STRIP) -s tmux-cpu-freq

tmux-ping: tmux-ping.c
	$(CC) $(CFLAGS) -o tmux-ping tmux-ping.c
	$(STRIP) -s tmux-ping

clean:
	rm -f tmux-cpu-freq	
	rm -f tmux-ping	

install: tmux-cpu-freq tmux-ping
	install -D tmux-cpu-freq $(INSTALL_DIR)/tmux-cpu-freq
	install -D tmux-ping $(INSTALL_DIR)/tmux-ping
