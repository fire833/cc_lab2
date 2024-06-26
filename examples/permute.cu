
#include <cstdio>
#include <cstdlib>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

const int arg_count = 12;
static const int mask_host[12] = { 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 }; 

int *parse_input(char* input, int parsed_len) {
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
__global__ void permute_array(int *in, int* mask, int *out) {
    unsigned int index = blockDim.x * blockIdx.x + threadIdx.x;
    if (index < arg_count) {
        out[mask[index]] = in[index];
    }
}

__host__ int main(int argc, char **argv) {
    if (argc != 3) {
		printf("{\"error\": \"2 arguments required, the program call name, the number of values (as an integer), and the list of values, comma separated.\",\"code\":1}");
		exit(1);
	}

	char *none;
  	int input_int_len = strtol(argv[1], &none, 10);

	if (input_int_len != arg_count) {
		printf("{\"error\": \"must provide as many inputs as there are arguments (%d)\",\"code\":1}", arg_count);
		exit(1);
	}

    int *input_host = parse_input(argv[2], input_int_len);
    int *output_host = (int*)calloc(input_int_len, sizeof(int));
    int *input_gpu;
    int *mask_gpu;
    int *output_gpu;

    cudaMalloc((void**) &input_gpu, input_int_len * sizeof(int));
    cudaMalloc((void**) &mask_gpu, input_int_len * sizeof(int));
    cudaMalloc((void**) &output_gpu, input_int_len * sizeof(int));

    // Copy data to device
    cudaMemcpy(input_gpu, input_host, input_int_len * sizeof(int), cudaMemcpyHostToDevice);
    // Copy mask to device
    cudaMemcpy(mask_gpu, mask_host, input_int_len * sizeof(int), cudaMemcpyHostToDevice);
    
    clock_t start, end;
    
	start = clock();
    permute_array<<<1, input_int_len>>>(input_gpu, mask_gpu, output_gpu);
    cudaDeviceSynchronize();
	end = clock();

    // Copy data back over
    cudaMemcpy(output_host, output_gpu, input_int_len * sizeof(int), cudaMemcpyDeviceToHost);

	printf("{\"values\": [");
	for (int i = 0; i < input_int_len; i++) {
		if (i == input_int_len - 1) {
			printf("%d],", output_host[i]);
		} else {
    		printf("%d,", output_host[i]);
		}
  	}

	printf("\"compute\": %.1f, \"code\": 0}\\n", ((double) (end - start)));

    cudaFree(&input_gpu);
    cudaFree(&output_gpu);
    cudaFree(&mask_gpu);
    free(input_host);
    free(output_host);
}
