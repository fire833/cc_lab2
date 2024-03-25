#include <arm_neon.h>
#include <stdio.h>

/*

gcc -g -o3 neon2.c -o exe_neon2_o3
objdump -d exe_neon2_o3 > disasm_neon2_o3
*/

void permute(const uint8_t* input, uint8_t* output, const uint8x16_t indices) {
    uint8x16_t myInput = vld1q_u8(input); //load input
    uint8x16_t myPermute = vqtbl1q_u8(myInput, indices); //permute by table lookup
    vst1q_u8(output, myPermute); //load to output
}

void printArr(const uint8_t* output){

    for (int i = 0; i < 16; ++i) {
        printf("%d ", output[i]);
    }
    printf("\n");
}

int main() {
    const uint8_t indicies[16] = {15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0};
	// const uint8_t indicies2[16] = {5,4,3,2,1,0,10,9,8,7,6,15,14,13,12,11};
    const uint8_t input[16] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
    uint8_t output[16];

    permute(input, output, vld1q_u8(indicies));
    printArr(output);

    return 0;
}
