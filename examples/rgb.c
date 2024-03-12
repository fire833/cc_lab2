

#include <stdio.h>
#include <arm_neon.h>

/*
    Running

    gcc -g -o3 rgb.c -o exe_rgb_o3
    objdump -d exe_rgb_o3 > disasm_rgb_o3
*/


void rgb_deinterleave_c(uint8_t *r, uint8_t *g, uint8_t *b, uint8_t *rgb, int len_color); 

int main(){

    // Example input data
    uint8_t rgb_data[] = {
        255, 0, 0,   // First pixel: red
        0, 255, 0,   // First pixel: green
        0, 0, 255,   // First pixel: blue
        128, 128, 128,  // Second pixel: gray
        // Add more pixels as needed
    };
    int len_color = sizeof(rgb_data) / 3; // Length of one color channel

    // Output arrays for deinterleaved colors
    uint8_t r[len_color];
    uint8_t g[len_color];
    uint8_t b[len_color];

    rgb_deinterleave_c(r, g, b, rgb_data, len_color);

    return 0;
}

void rgb_deinterleave_c(uint8_t *r, uint8_t *g, uint8_t *b, uint8_t *rgb, int len_color) {
        /*
     * Take the elements of "rgb" and store the individual colors "r", "g", and "b"
     */
    int num8x16 = len_color / 16;
    uint8x16x3_t intlv_rgb;
    for (int i=0; i < num8x16; i++) {
        intlv_rgb = vld3q_u8(rgb+3*16*i);
        vst1q_u8(r+16*i, intlv_rgb.val[0]);
        vst1q_u8(g+16*i, intlv_rgb.val[1]);
        vst1q_u8(b+16*i, intlv_rgb.val[2]);
    }
}