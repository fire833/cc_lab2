
template = {
	"compiler_prefix": ["nvcc", "-O3"],
	"program_output": "prog.cu",
	"name": "cuda2",

	"template": """
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

const int arg_count = {{ arg_count }};
__device__ __constant__ int mask[{{ arg_count }}] = { {% for val in values %}{{val[1]}}{% if not values|last == val %}, {% endif %}{% endfor %} }; 

inline int *parse_input(char* input, int parsed_len) {
	int index = 0;
	int *output = (int *)calloc(parsed_len, sizeof(int));
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
      		output[index] = sum;
      		sum = 0;
      		index++;
     		continue;
    	}
    	}
  	}

  	output[index] = sum;
	return output;
}

// Kernel to permute values
__global__ void permute_array(int *in, int *out) {
    unsigned int index = blockDim.x * blockIdx.x + threadIdx.x;
    if (index < arg_count) {
        out[mask[index]] = in[index];
    }
}

__host__ int main(int argc, char **argv) {
    if (argc != 3) {
		printf("{\\"error\\": \\"2 arguments required, the program call name, the number of values (as an integer), and the list of values, comma separated.\\",\\"code\\":1}");
		exit(1);
	}

	char *none;
  	int input_int_len = strtol(argv[1], &none, 10);

	if (input_int_len != arg_count) {
		printf("{\\"error\\": \\"must provide as many inputs as there are arguments (%d vs %d provided)\\",\\"code\\":1}", arg_count, input_int_len);
		exit(1);
	}

    int *input_host = parse_input(argv[2], input_int_len);
    int *output_host = (int*)calloc(input_int_len, sizeof(int));
    int *input_gpu;
    int *output_gpu;

    cudaMalloc((void**) &input_gpu, input_int_len * sizeof(int));
    cudaMalloc((void**) &output_gpu, input_int_len * sizeof(int));

    // Copy data to device
    cudaMemcpy(input_gpu, input_host, input_int_len * sizeof(int), cudaMemcpyHostToDevice);
    
    cudaEvent_t start, stop;
  	cudaEventCreate(&start);
  	cudaEventCreate(&stop);
    
	cudaEventRecord(start);
    
	permute_array<<<1, input_int_len>>>(input_gpu, output_gpu);
	
	cudaEventRecord(stop);
    cudaEventSynchronize(stop);
  	float milliseconds = -1;
  	cudaEventElapsedTime(&milliseconds, start, stop);

    // Copy data back over
    cudaMemcpy(output_host, output_gpu, input_int_len * sizeof(int), cudaMemcpyDeviceToHost);

	printf("{\\"values\\": [");
	for (int i = 0; i < input_int_len; i++) {
		if (i == input_int_len - 1) {
			printf("%d],", output_host[i]);
		} else {
    		printf("%d,", output_host[i]);
		}
  	}

	printf("\\"compute\\": %d, \\"code\\": 0}\\n", (int) (milliseconds * 1000000));

    cudaFree(input_gpu);
    cudaFree(output_gpu);
    free(input_host);
    free(output_host);
}

"""
}
