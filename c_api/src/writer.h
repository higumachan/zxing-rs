//
// Created by 日熊悠太 on 2019-05-27.
//

#ifndef ZXING_RS_WRITER_H
#define ZXING_RS_WRITER_H

#include <cstdlib>

extern "C" {
    int zxing_write_qrcode(const char *text, uint8_t **buffer, int format, int width, int height, int margin,
                           int ecc_level);
    int zxing_write_release_buffer(uint8_t* buffer);
};
#endif //ZXING_RS_WRITER_H
