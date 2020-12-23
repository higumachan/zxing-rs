//
// Created by 日熊悠太 on 2019-05-20.
//

#include <cstdlib>
#include "reader.h"
#include <DecodeHints.h>
#include <Result.h>
#include "MultiFormatReader.h"
#include "GenericLuminanceSource.h"
#include "HybridBinarizer.h"
#include "TextUtfEncoding.h"
#include <cstring>
#include <iostream>

using Binarizer = ZXing::HybridBinarizer;

int
zxing_read_qrcode(ZXING_RESULT **result, const uint8_t *buffer, int width, int height, int row_bytes, int pixel_bytes,
                  int index_r,
                  int index_g, int index_b)
{
    ZXing::DecodeHints hints;
    hints.setShouldTryHarder(true);
    ZXing::MultiFormatReader reader(hints);

    ZXing::GenericLuminanceSource source((int)width, (int)height, buffer, row_bytes, pixel_bytes, index_r, index_g, index_b);
    Binarizer binImage(std::shared_ptr<ZXing::LuminanceSource>(&source, [](void*) {}));

    ZXing::Result reader_result = reader.read(binImage);



    *result = (ZXING_RESULT *)malloc(sizeof(ZXING_RESULT));
    (*result)->num_bits = reader_result.numBits();
    (*result)->status = static_cast<int>(reader_result.status());
    (*result)->bytes = nullptr;

    if (reader_result.status() != ZXing::DecodeStatus::NoError) {
        return -1;
    }

    (*result)->format = static_cast<int>(reader_result.format());
    auto s = ZXing::TextUtfEncoding::ToUtf8(reader_result.text());
    (*result)->bytes = (uint8_t *)malloc(sizeof(char) * s.size());
    std::memcpy((*result)->bytes, s.data(), sizeof(char) * s.size());
    (*result)->bytes_size = sizeof(char) * s.size();

    return 0;
}

int release_result(ZXING_RESULT *result)
{
    if (result->bytes != nullptr) {
        free(result->bytes);
    }
    free(result);
    return 0;
}
