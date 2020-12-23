use image::DynamicImage;
use image::GenericImageView;
use num::FromPrimitive;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_int;
use std::slice::from_raw_parts;
use std::str::from_utf8;

#[macro_use]
extern crate bitflags;

bitflags! {
    #[allow(non_camel_case_types)]
    pub struct Format: u32 {
        /** Aztec 2D barcode format. */
        const AZTEC = (1 << 0);

        /** CODABAR 1D format. */
        const CODABAR = (1 << 1);

        /** Code 39 1D format. */
        const CODE_39 = (1 << 2);

        /** Code 93 1D format. */
        const CODE_93 = (1 << 3);

        /** Code 128 1D format. */
        const CODE_128 = (1 << 4);

        /** Data Matrix 2D barcode format. */
        const DATA_MATRIX = (1 << 5);

        /** EAN-8 1D format. */
        const EAN_8 = (1 << 6);

        /** EAN-13 1D format. */
        const EAN_13 = (1 << 7);

        /** ITF (Interleaved Two of Five) 1D format. */
        const ITF = (1 << 8);

        /** MaxiCode 2D barcode format. */
        const MAXICODE = (1 << 9);

        /** PDF417 format. */
        const PDF_417 = (1 << 10);

        /** QR Code 2D barcode format. */
        const QR_CODE = (1 << 11);

        /** RSS 14 */
        const RSS_14 = (1 << 12);

        /** RSS EXPANDED */
        const RSS_EXPANDED = (1 << 13);

        /** UPC-A 1D format. */
        const UPC_A = (1 << 14);

        /** UPC-E 1D format. */
        const UPC_E = (1 << 15);

        /** UPC/EAN extension (1D). Not a stand-alone format. */
        const UPC_EAN_EXTENSION = (1 << 16);
    }
}

impl Into<c_int> for Format {
    fn into(self) -> c_int {
        self.bits() as c_int
    }
}

impl From<c_int> for Format {
    fn from(value: c_int) -> Format {
        Format::from_bits_truncate(value as u32)
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

#[derive(Debug, PartialEq)]
pub enum EncodeError {
    FormatError,
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
    fn zxing_read_qrcode(
        result: *mut *mut ZxingResult,
        buffer: *const u8,
        width: c_int,
        height: c_int,
        row_bytes: c_int,
        pixel_bytes: c_int,
        index_r: c_int,
        index_g: c_int,
        index_b: c_int,
    ) -> c_int;
    fn release_result(result: *mut ZxingResult);

    fn zxing_write_qrcode(
        text: *const i8,
        buffer: *mut *mut u8,
        format: c_int,
        width: c_int,
        height: c_int,
        margin: c_int,
        ecc_level: c_int,
    ) -> c_int;
    fn zxing_write_release_buffer(buffer: *mut u8) -> c_int;
}

pub fn read_qrcode(image: DynamicImage) -> Result<DecodedQrCode, DecodeError> {
    let image_buf = image.to_rgb8();
    let mut result = mem::MaybeUninit::<*mut ZxingResult>::uninit();
    unsafe {
        let ret_code = zxing_read_qrcode(
            result.as_mut_ptr(),
            image_buf.as_ptr(),
            image.width() as crate::c_int,
            image.height() as crate::c_int,
            (image.width() * 3) as crate::c_int,
            3,
            0,
            1,
            2,
        );
        let result = result.assume_init();

        if ret_code != 0 {
            return Err(DecodeError::from_i32((*result).status).unwrap());
        }

        let s = from_raw_parts((*result).bytes, (*result).bytes_size as usize);
        let text = from_utf8(s).unwrap();

        Ok(DecodedQrCode {
            raw_result: result,
            text: text.to_string(),
            format: (*result).format.into(),
        })
    }
}

pub fn write_qrcode(
    text: &str,
    format: Format,
    width: u64,
    height: u64,
    mergin: u64,
    ecc_level: u64,
) -> Result<Vec<u8>, EncodeError> {
    let mut buffer_vector = Vec::new();
    unsafe {
        let mut buffer: *mut u8 = mem::uninitialized();
        let buffer_size = zxing_write_qrcode(
            CString::new(text).unwrap().as_ptr(),
            &mut buffer,
            format.into(),
            width as c_int,
            height as c_int,
            mergin as c_int,
            ecc_level as c_int,
        );
        println!("{}", buffer_size);
        buffer_vector.resize(buffer_size as usize, 0);
        buffer.copy_to(buffer_vector.as_mut_ptr(), buffer_size as usize);
    }

    return Ok(buffer_vector);
}

#[cfg(test)]
mod tests {
    extern crate image;

    use crate::{read_qrcode, DecodeError, Format, ZxingResult};
    use image::open;
    use image::png::PNGEncoder;
    use image::ColorType;
    use image::GenericImageView;
    use std::env;
    use std::fs::File;
    use std::mem;
    use std::path::Path;
    use std::slice::from_raw_parts;
    use std::str::from_utf8;
    use tempdir::TempDir;

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
            let ret = crate::zxing_read_qrcode(
                &mut result,
                image_buf.as_ptr(),
                image.width() as crate::c_int,
                image.height() as crate::c_int,
                (image.width() * 4) as crate::c_int,
                4,
                0,
                1,
                2,
            );
            let s = from_raw_parts((*result).bytes, (*result).bytes_size as usize);
            let text = from_utf8(s);

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

    #[test]
    fn test_write_qrcode() {
        let td = TempDir::new("test_dir").unwrap();
        let text = "nadeko is cute";

        let buf = crate::write_qrcode(text, crate::Format::QR_CODE, 200, 200, 10, 0);

        assert!(buf.is_ok());

        let path = td.path().join("test.png");
        let output = File::create(&path).unwrap();

        let encoder = PNGEncoder::new(output);
        let res = encoder.encode(
            buf.unwrap().as_slice(),
            200 as u32,
            200 as u32,
            ColorType::L8,
        );

        assert!(res.is_ok());

        let image = open(&path).unwrap();

        let result = crate::read_qrcode(image).unwrap();
        assert_eq!(result.text, "nadeko is cute");
        assert_eq!(result.format, crate::Format::QR_CODE);
    }
}
