/*
 * tmux-ping.c - a little tmux "applet" for showing whether we can ping hosts
 *
 * by Lucas Martin-King
 *
 * [ Licenced under the GPLv2 ]
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include <fcntl.h>

#define PING_COUNT	"1"	/* must be a string! */
#define PING_TIMEOUT	"1"	/* must be a string! */

#define PING_PROG	"/bin/ping"

#define DEFAULT_HOST	"127.0.0.1"

#define COLOUR_OKAY	"green"
#define COLOUR_ERROR	"red"

int main(int argc, char **argv)
{
	char *host;
	int status;
	pid_t pid;

	if (argc > 1) {
		host = argv[1];
	} else {
		host = DEFAULT_HOST;
	}

	pid = fork();

	if (pid == 0) {
		int nullfd = open("/dev/null", O_RDWR);

		dup2(nullfd, 0);
		dup2(nullfd, 1);
		dup2(nullfd, 2);

		execl(PING_PROG, PING_PROG, "-c", PING_COUNT, "-w", PING_TIMEOUT, host, NULL);
	} else if (pid < 0) {
		printf("ER\n");
		return 1;
	} else {
		int retval;
		char *colour = NULL;

		waitpid(pid, &status, 0);
		retval = WEXITSTATUS(status);

		if (retval == 0) {
			colour = COLOUR_OKAY;
		} else {
			colour = COLOUR_ERROR;
		}

		printf("#[bg=%s]  #[default]\n", colour);
	}

	fflush(stdout);

	return 0;
}
