/*
 * tmux-cpu-freq.c - a little tmux "applet" for showing cpu frequencies
 *
 * by Lucas Martin-King
 *
 * [ Licenced under the GPLv2 ]
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* Set this to the number of CPU's your system */
#define NUMBER_CPUS	6

/* If your CPU can scale to more than 6 frequency levels,
   then you should change this */
#define MAX_CPU_LEVELS	6

/* And you can add colours to this, if needed */
#define MAX_LEVEL_COLORS 6
static const char *level_colors[MAX_LEVEL_COLORS] = {
	"red",
	"yellow",
	"green",
	"blue",
	"cyan",
	"magenta",
};

#define CPU_FREQ		"/sys/devices/system/cpu/"
#define FREQ_CUR_FORMAT		CPU_FREQ "cpu/cpu%d/cpufreq/scaling_cur_freq"
#define FREQ_AVAILABLE_PATH	CPU_FREQ "cpu0/cpufreq/scaling_available_frequencies"

static unsigned int cpu_freq(unsigned int cpu)
{
	#define PATH_LEN 64
	char path[PATH_LEN];
	FILE *f;
	int ret = 0;
	unsigned int freq = 0;

	snprintf(path, PATH_LEN, FREQ_CUR_FORMAT, cpu);

	f = fopen(path, "r");

	if (f) {
		ret = fscanf(f, "%u\n", &freq);
		fclose(f);
	}

	if (ret == 1)
		return freq;
	else
		return 0;
}

static unsigned int cpu_levels(unsigned int max_levels, unsigned int *levels)
{
	FILE *f;

	unsigned int nr_levels = 0;
	unsigned int freq = 0;

	f = fopen(FREQ_AVAILABLE_PATH, "r");

	if (f) {
		while (fscanf(f, "%u ", &freq) == 1) {
			if (nr_levels < max_levels) {
				*levels = freq;

				levels++;
				nr_levels++;
			} else {
				break;
			}

			freq = 0;
		}

		fclose(f);
	}

	return nr_levels;
}

int main(int argc, char **argv)
{
	unsigned int cpu;
	unsigned int freq;

	unsigned int nr_cpus = NUMBER_CPUS;
	unsigned int nr_levels;

	unsigned int levels[MAX_CPU_LEVELS];
	unsigned int level;

	const char *colour = NULL;

	if (argc > 1) {
		int n = strtol(argv[1], (char **)NULL, 10); 

		if (n > 0) {
			nr_cpus = n;
		}
	}

	nr_levels = cpu_levels(MAX_CPU_LEVELS, levels);

	for (cpu = 0; cpu < nr_cpus; cpu++) {
		freq = cpu_freq(cpu);	

		for (level = 0; level < nr_levels; level++) {
			if (levels[level] == freq) {
				break;
			}
		}

		if (level < MAX_LEVEL_COLORS) {
			colour = level_colors[level];
		}

		printf("#[bg=%s]  ", colour);
	}

	printf("#[default]\n");
	fflush(stdout);

	return 0;
}
