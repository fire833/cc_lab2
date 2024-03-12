#include "immintrin.h"
#include <stdio.h>

void permute(float *dst, float *src, const int* shuffleMask) {
    __m256 v = _mm256_loadu_ps(&src[0]); // Load the first 8 values from src
    __m256 v_dst;

    // Permute the values according to the shuffle mask
    v_dst = _mm256_permutevar8x32_ps(v, _mm256_loadu_si256((__m256i*)shuffleMask));

    // Store the permuted values to dst
    _mm256_storeu_ps(&dst[0], v_dst);

	printf("\nprinting variables:");
	for (int i = 0; i < 8; i++) {
		printf("\n%.0f",dst[i]);
	}

}

int main() {
	float src[8] = {1, 2, 3, 4, 5, 6, 7, 8}; // Source array
    float dst[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    const int shuffleMask[8] = {7, 6, 5, 4, 3, 2, 1, 0}; // Shuffle mask

	permute(dst, src, shuffleMask);

	return 0;
}
