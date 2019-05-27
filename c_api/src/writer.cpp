//
// Created by 日熊悠太 on 2019-05-27.
//

#include <cstdlib>
#include <iostream>
#include "writer.h"
#include "BarcodeFormat.h"
#include "MultiFormatWriter.h"
#include "BitMatrix.h"
#include "TextUtfEncoding.h"
#include "ZXStrConvWorkaround.h"

int zxing_write_qrcode(const char* text, uint8_t ** buffer, int format, int width, int height, int margin, int ecc_level)
{
    ZXing::MultiFormatWriter writer(static_cast<ZXing::BarcodeFormat>(format));
    writer.setMargin(margin);
    writer.setEccLevel(ecc_level);

    auto matrix = writer.encode(ZXing::TextUtfEncoding::FromUtf8(text), width, height);
    int size = matrix.width() * matrix.height();
    *buffer = (unsigned char*)std::calloc(matrix.width() * matrix.height(), sizeof(unsigned char));
    const unsigned char black = 0;
    const unsigned char white = 255;

    for (int y = 0; y < matrix.height(); ++y) {
        for (int x = 0; x < matrix.width(); ++x) {
            (*buffer)[y * matrix.width() + x] = matrix.get(x, y) ? black : white;
        }
    }

    return size;
}


int zxing_write_release_buffer(unsigned char* buffer)
{
    std::free(buffer);
    return 0;
}
