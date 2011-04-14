#include <stdio.h>
#include <string.h>

#include <stdlib.h>

/* And you can add colours to this, if needed */
#define MAX_LEVEL_COLOURS 6
static const char *level_colours[MAX_LEVEL_COLOURS] = {
	"red",
	"yellow",
	"green",
	"blue",
	"cyan",
	"magenta",
};

int extract(char *s, size_t len)
{
	char *p;
	int start_pos = 0;
	int end_pos = 0;
	char buf[255];

	for (p = s; *p != ':'; p++) {
		start_pos++;
	}

	char *start = strchr(s, ':'); start++;

	start_pos++;

	for (p = s + start_pos; *p == ' '; p++) {
		start_pos++;
	}

	end_pos = start_pos;

	for (p = s + start_pos; *p != 'k'; p++) {
		end_pos++;
	}

	p = s + start_pos;
	strncpy(buf, p, end_pos - start_pos);

	return atoi(buf);
}

int main(int argc, char **argv)
{
	#define LEN 255

	FILE *f = fopen("/proc/meminfo", "r");

	if (f == NULL) {
		printf("ER\n");
		return 1;
	}

	char *mem_free = (char *) malloc(LEN + 1);
	char *mem_total = (char *) malloc(LEN + 1);
	char *mem_buffers = (char *) malloc(LEN + 1);
	char *mem_cached = (char *) malloc(LEN + 1);

	size_t length = LEN;

	getline(&mem_total, &length, f);
	getline(&mem_free, &length, f);
	getline(&mem_buffers, &length, f);
	getline(&mem_cached, &length, f);

	fclose(f);

	int mtotal = extract(mem_total, length);
	int mfree = extract(mem_free, length);
	int mbuffers = extract(mem_buffers, length);
	int mcached = extract(mem_cached, length);

	int mtfree = mfree + mbuffers + mcached;

	int pfree = (int)((float)mtfree / (float)mtotal * 100.0 );

	int level;
	int level_amt = (int)(100.0 / (float)MAX_LEVEL_COLOURS);

	for (level = MAX_LEVEL_COLOURS - 1; level >= 0; level--) {
		if (pfree > (level * level_amt)) {
			break;
		}
	}

	printf("#[bg=%s]  #[default]\n", level_colours[level]);

	free(mem_total);
	free(mem_free);
	free(mem_buffers);
	free(mem_cached);

	return 0;
}
