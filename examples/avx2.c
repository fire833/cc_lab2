
#include <immintrin.h>
#include <stdio.h>

void permute() {
  __m256i values = {0xefff0001efff0002, 0xefff0003efff0004, 0xefff0005efff0006,
                    0xefff0007efff0008};
  __m256i mask = {0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff,
                  0xffffffffffffffff};
  // __m256i sol = _mm256_permutevar8x32_epi32(values, mask);

  int res[8];
  _mm256_maskstore_epi32(res, mask, values);
  // _mm256_storeu2_m128i(res, res, sol);
  for (int i = 0; i < 8; i++) {
    printf("%d\n", res[i]);
  }
}

int main() { permute(); }
