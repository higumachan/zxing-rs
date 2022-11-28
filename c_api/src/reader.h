//
// Created by 日熊悠太 on 2019-05-20.
//

#ifndef ZXING_CPP_READER_H
#define ZXING_CPP_READER_H

#include <cstdint>

typedef struct ZxingQrResult {
    int status;
    int num_bits;
    int format;
    uint8_t* bytes;
    int bytes_size;
    int corners[8];
} ZXING_RESULT;


extern "C" {
    int zxing_read_qrcode(ZXING_RESULT **result, const uint8_t *buffer, int width, int height, int row_bytes, int pixel_bytes);
    int release_result(ZXING_RESULT* result);
}

#endif //ZXING_CPP_READER_H
