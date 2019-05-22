use std::os::raw::c_int;
use image::DynamicImage;
use std::slice::from_raw_parts;
use std::str::from_utf8;
use std::mem;
use image::GenericImageView;
use num::FromPrimitive;


#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Format {
    /** Aztec 2D barcode format. */
    AZTEC,

    /** CODABAR 1D format. */
    CODABAR,

    /** Code 39 1D format. */
    CODE_39,

    /** Code 93 1D format. */
    CODE_93,

    /** Code 128 1D format. */
    CODE_128,

    /** Data Matrix 2D barcode format. */
    DATA_MATRIX,

    /** EAN-8 1D format. */
    EAN_8,

    /** EAN-13 1D format. */
    EAN_13,

    /** ITF (Interleaved Two of Five) 1D format. */
    ITF,

    /** MaxiCode 2D barcode format. */
    MAXICODE,

    /** PDF417 format. */
    PDF_417,

    /** QR Code 2D barcode format. */
    QR_CODE,

    /** RSS 14 */
    RSS_14,

    /** RSS EXPANDED */
    RSS_EXPANDED,

    /** UPC-A 1D format. */
    UPC_A,

    /** UPC-E 1D format. */
    UPC_E,

    /** UPC/EAN extension format. Not a stand-alone format. */
    UPC_EAN_EXTENSION,
}


impl FromPrimitive for Format {
    fn from_i64(n: i64) -> Option<Self> {
        if n < 0 {
            return None;
        }
        Self::from_u64(n as u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Format::AZTEC),
            1 => Some(Format::CODABAR),
            2 => Some(Format::CODE_39),
            3 => Some(Format::CODE_93),
            4 => Some(Format::CODE_128),
            5 => Some(Format::DATA_MATRIX),
            6 => Some(Format::EAN_8),
            7 => Some(Format::EAN_13),
            8 => Some(Format::ITF),
            9 => Some(Format::MAXICODE),
            10 => Some(Format::PDF_417),
            11 => Some(Format::QR_CODE),
            12 => Some(Format::RSS_14),
            13 => Some(Format::RSS_EXPANDED),
            14 => Some(Format::UPC_A),
            15 => Some(Format::UPC_E),
            16 => Some(Format::UPC_EAN_EXTENSION),
            _ => None,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum DecodeError {
    NotFound = 1,
    FormatError,
    ChecksumError,
}

impl FromPrimitive for DecodeError {
    fn from_i64(n: i64) -> Option<Self> {
        if n < 0 {
            return None;
        }
        Self::from_u64(n as u64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            1 => Some(DecodeError::NotFound),
            2 => Some(DecodeError::FormatError),
            3 => Some(DecodeError::ChecksumError),
            _ => None,
        }
    }
}


pub struct DecodedQrCode {
    raw_result: *mut ZxingResult,
    pub text: String,
    pub format: Format,
}

impl Drop for DecodedQrCode {
    fn drop(&mut self) {
        unsafe {
            release_result(self.raw_result);
        }
    }
}

#[repr(C)]
struct ZxingResult {
    status: c_int,
    num_bits: c_int,
    format: c_int,
    bytes: *mut u8,
    bytes_size: c_int,
}

#[link(name = "zxing_c_api", kind = "static")]
extern "C" {
    fn zxing_read_qrcode(result: *mut *mut ZxingResult, buffer: *const u8, width: c_int, height: c_int, row_bytes: c_int, pixel_bytes: c_int, index_r: c_int, index_g: c_int, index_b: c_int) -> c_int;
    fn release_result(result: *mut ZxingResult);
}

pub fn read_qrcode(image: DynamicImage) -> Result<DecodedQrCode, DecodeError> {
    let image_buf = image.to_rgb();
    unsafe {
        let mut result: *mut ZxingResult = mem::uninitialized();
        let ret_code = crate::zxing_read_qrcode(&mut result, image_buf.as_ptr(),
                                           image.width() as crate::c_int, image.height() as crate::c_int,
                                           (image.width() * 3) as crate::c_int,
                                           3, 0, 1, 2);

        if ret_code != 0 {
            return Err(DecodeError::from_i32((*result).status).unwrap());
        }

        let s = from_raw_parts((*result).bytes, (*result).bytes_size as usize);
        let text = from_utf8(s).unwrap();

        Ok(DecodedQrCode{ raw_result: result, text: text.to_string(), format: Format::from_i32((*result).format).unwrap()})
    }
}

#[cfg(test)]
mod tests {
    extern crate image;

    use std::mem;
    use image::open;
    use image::GenericImageView;
    use std::path::Path;
    use crate::{ZxingResult, read_qrcode, Format, DecodeError};
    use std::env;
    use std::slice::from_raw_parts;
    use std::str::from_utf8;

    #[test]
    fn test_read() {
        let path = Path::new("./image/01-1.png");
        println!("{}", path.display());
        let image = open(path);
        if let Err(err) = &image {
            println!("{:?}", err);
        }
        let image = image.unwrap();
        let image_buf = image.to_rgba();
        unsafe {
            let mut result: *mut ZxingResult = mem::uninitialized();
            let ret = crate::zxing_read_qrcode(&mut result,image_buf.as_ptr(),
                                               image.width() as crate::c_int,image.height() as crate::c_int,
                                               (image.width() * 4) as crate::c_int,
                                               4 ,0, 1, 2);
            let s = from_raw_parts((*result).bytes, (*result).bytes_size as usize);
            let text = from_utf8(s) ;

            assert_eq!(ret, 0);
            assert!(text.is_ok());
            assert_eq!(text.unwrap(), "http://www.amazon.co.jp/gp/aw/rd.html?uid=NULLGWDOCOMO&url=/gp/aw/h.html&at=aw_intl6-22");
        }
    }

    #[test]
    fn test_read_qrcode() {
        let path = Path::new("./image/01-1.png");
        let image = open(path).unwrap();

        let result = crate::read_qrcode(image).unwrap();
        assert_eq!(result.text, "http://www.amazon.co.jp/gp/aw/rd.html?uid=NULLGWDOCOMO&url=/gp/aw/h.html&at=aw_intl6-22");
        assert_eq!(result.format, crate::Format::QR_CODE);
    }

    #[test]
    fn test_nodetect_error() {
        let path = Path::new("./image/nadeko1.jpg");
        let image = open(path).unwrap();

        let result = crate::read_qrcode(image);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), DecodeError::NotFound);
    }
}
