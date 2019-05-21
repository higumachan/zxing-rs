use std::os::raw::c_int;
use image::DynamicImage;
use std::slice::from_raw_parts;
use std::str::from_utf8;
use std::mem;
use image::GenericImageView;


pub struct DecodedQrCode {
    raw_result: *mut ZxingResult,
    pub text: String,
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

pub fn read_qrcode(image: DynamicImage) -> Result<DecodedQrCode, String> {
    let image_buf = image.to_rgb();
    unsafe {
        let mut result: *mut ZxingResult = mem::uninitialized();
        let ret = crate::zxing_read_qrcode(&mut result, image_buf.as_ptr(),
                                           image.width() as crate::c_int, image.height() as crate::c_int,
                                           (image.width() * 3) as crate::c_int,
                                           3, 0, 1, 2);
        let s = from_raw_parts((*result).bytes, (*result).bytes_size as usize);
        let text = from_utf8(s).unwrap();

        return Ok(DecodedQrCode{ raw_result: result, text: text.to_string()})
    }
}

#[cfg(test)]
mod tests {
    extern crate image;

    use std::mem;
    use image::open;
    use image::GenericImageView;
    use std::path::Path;
    use crate::{ZxingResult, read_qrcode};
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

        let result = crate::read_qrcode(image);
        assert_eq!(result.unwrap().text, "http://www.amazon.co.jp/gp/aw/rd.html?uid=NULLGWDOCOMO&url=/gp/aw/h.html&at=aw_intl6-22");
    }
}
