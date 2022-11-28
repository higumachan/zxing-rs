//
// Created by 日熊悠太 on 2019-05-20.
//

#include <cstdlib>
#include <cstring>
#include <iostream>
#include "reader.h"
#include "DecodeHints.h"
#include "Result.h"
#include "MultiFormatReader.h"
#include "HybridBinarizer.h"
#include "TextUtfEncoding.h"
#include "ReadBarcode.h"

using Binarizer = ZXing::HybridBinarizer;

int
zxing_read_qrcode(ZXING_RESULT **result, const uint8_t *buffer, int width, int height, int row_bytes, int pixel_bytes)
{
    ZXing::DecodeHints hints;
    hints.setTryHarder(true);
    hints.setBinarizer(ZXing::Binarizer::LocalAverage);
    ZXing::MultiFormatReader reader(hints);

    ZXing::ImageView image { buffer, width, height, ZXing::ImageFormat::RGB, row_bytes, pixel_bytes };

    ZXing::Result reader_result = ZXing::ReadBarcode(image, hints);


    *result = (ZXING_RESULT *)malloc(sizeof(ZXING_RESULT));
    (*result)->num_bits = reader_result.numBits();
    (*result)->status = static_cast<int>(reader_result.status());
    (*result)->bytes = nullptr;

    if (reader_result.status() != ZXing::DecodeStatus::NoError) {
        return -1;
    }

    (*result)->format = static_cast<int>(reader_result.format());
    auto s = reader_result.text();
    (*result)->bytes = (uint8_t *)malloc(sizeof(char) * s.size());
    std::memcpy((*result)->bytes, s.data(), sizeof(char) * s.size());
    (*result)->bytes_size = sizeof(char) * s.size();

    ZXing::Quadrilateral<ZXing::PointT<int>> pos = reader_result.position();
    (*result)->corners[0] = pos.topLeft().x;
    (*result)->corners[1] = pos.topLeft().y;
    (*result)->corners[2] = pos.topRight().x;
    (*result)->corners[3] = pos.topRight().y;
    (*result)->corners[4] = pos.bottomRight().x;
    (*result)->corners[5] = pos.bottomRight().y;
    (*result)->corners[6] = pos.bottomLeft().x;
    (*result)->corners[7] = pos.bottomLeft().y;
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
