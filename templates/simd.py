
template = {
	"compiler_prefix": ["clang", "-O3", "-Wall", "-mavx", "-mavx2", "-msse", "-g"],
	"program_output": "prog.c",

	"template": """
#include <immintrin.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

const int arg_count = {{ arg_count }};
// static const __m128i quadmask = {0xffffffffffffffff, 0xffffffffffffffff};
// static const __m256i octmask = {0xffffffffffffffff, 0xffffffffffffffff,
//                                 0xffffffffffffffff, 0xffffffffffffffff};

inline float *parse_input(char* input, int parsed_len) {
	int index = 0;
	float *output = (float *)calloc(parsed_len, sizeof(float));
	int sum = 0;
  	for (int i = 0; i < strlen(input); i++) {
    	// Shift over the sum and add a new value, whatever it may be.
    	switch (input[i]) {
    	case '0':
      		sum = 10 * sum;
      		continue;
    	case '1':
      		sum = 10 * sum + 1;
      		continue;
    	case '2':
      		sum = 10 * sum + 2;
      		continue;
    	case '3':
      		sum = 10 * sum + 3;
      		continue;
    	case '4':
      		sum = 10 * sum + 4;
      		continue;
    	case '5':
      		sum = 10 * sum + 5;
      		continue;
    	case '6':
      		sum = 10 * sum + 6;
      		continue;
    	case '7':
      		sum = 10 * sum + 7;
      		continue;
    	case '8':
      		sum = 10 * sum + 8;
      		continue;
    	case '9':
      		sum = 10 * sum + 9;
      		continue;

    	// We are at the end of the number, reset to a new sum.
    	case ',': {
      		output[index] = (float) sum;
      		sum = 0;
      		index++;
     		continue;
    	}
    	}
  	}

  	output[index] = (float) sum;
	return output;
}

inline void permute(float *in, float *out) {
{{ permute_gen }}
}

int main(int argc, char **argv) {
	if (argc != 3) {
		printf("{\\"error\\": \\"2 arguments required, the program call name, the number of values (as an integer), and the list of values, comma separated.\\",\\"code\\":1}");
		exit(1);
	}

	char *none;
  	int input_int_len = strtol(argv[1], &none, 10);

	if (input_int_len != arg_count) {
		printf("{\\"error\\": \\"must provide as many inputs as there are arguments (%d)\\",\\"code\\":1}", arg_count);
		exit(1);
	}

	float *input = parse_input(argv[2], input_int_len);
	float *output = (float*)calloc(input_int_len, sizeof(float));
	clock_t start, end;

	start = clock();
	permute(input, output);
	end = clock();

	printf("{\\"values\\": [");
	for (int i = 0; i < input_int_len; i++) {
		if (i == input_int_len - 1) {
			printf("%d],", (int) output[i]);
		} else {
    		printf("%d,", (int) output[i]);
		}
  	}

	printf("\\"compute\\": %.1f, \\"code\\": 0}\\n", ((double) (end - start)));

	free(input);
	free(output);
}

"""
}
