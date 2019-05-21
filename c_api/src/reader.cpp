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
#include <iostream>


using Binarizer = ZXing::HybridBinarizer;

int
zxing_read_qrcode(ZXING_RESULT **result, const uint8_t *buffer, int width, int height, int row_bytes, int pixel_bytes,
                  int index_r,
                  int index_g, int index_b)
{
    std::cout << std::hex;
    for (int i = 0; i < 20; i++) {
        std::cout << (int)buffer[i] << " ";
    }
    std::cout << std::endl;

#if 1
    ZXing::DecodeHints hints;
    hints.setShouldTryHarder(true);
    ZXing::MultiFormatReader reader(hints);

#endif
    ZXing::GenericLuminanceSource source((int)width, (int)height, buffer, row_bytes, pixel_bytes, index_r, index_g, index_b);
    Binarizer binImage(std::shared_ptr<ZXing::LuminanceSource>(&source, [](void*) {}));

    ZXing::Result reader_result = reader.read(binImage);
    std::cout << std::dec;
    std::cout << "Text:     " << ZXing::TextUtfEncoding::ToUtf8(reader_result.text()) << std::endl;
    std::cout << "Bytes.size:     " << std::dec << reader_result.rawBytes().size() << std::endl;
    std::cout << "Text.size:     " << std::dec << reader_result.text().size() << std::endl;
#if 1

    *result = (ZXING_RESULT *)malloc(sizeof(ZXING_RESULT));
    (*result)->num_bits = reader_result.numBits();
    (*result)->format = static_cast<int>(reader_result.format());
    (*result)->status = static_cast<int>(reader_result.status());
#if 0
    (*result)->bytes = (uint8_t *)malloc(sizeof(uint8_t) * reader_result.rawBytes().size());
    std::memcpy((*result)->bytes, reader_result.rawBytes().data(), sizeof(uint8_t) * reader_result.rawBytes().size());
    (*result)->bytes_size = reader_result.rawBytes().size();
#else
    auto s = ZXing::TextUtfEncoding::ToUtf8(reader_result.text());
    (*result)->bytes = (uint8_t *)malloc(sizeof(char) * s.size());
    std::memcpy((*result)->bytes, s.data(), sizeof(char) * s.size());
    (*result)->bytes_size = sizeof(char) * s.size();
#endif
#endif

    return 0;
}

int release_result(ZXING_RESULT *result)
{
    free(result->bytes);
    free(result);
    return 0;
}
