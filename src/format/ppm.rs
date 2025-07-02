use crate::traits::ImageRGB;
use crate::utils::{ByteIterator, byte_to_ascii};

pub fn read_ppm(file: &str) -> ImageRGB {
    let mut bytes = ByteIterator::from_file(file);
    let header = bytes.nextn(3);
    let header_ascii = byte_to_ascii(&header).trim_ascii_end();

    // ASCII magic number is P3
    // The binary variation is P6
    assert_eq!(
        header_ascii, "P3",
        "Invalid magic number in the header. Expected P3, got {}",
        header_ascii
    );

    // get width
    let mut width = String::from("");
    loop {
        let byte = bytes.nextn(1);
        let char = byte_to_ascii(&byte).trim();
        if !char.is_empty() {
            width += char
        } else {
            break;
        }
    }
    let width: usize = width.parse().unwrap();

    // get height
    let mut height = String::from("");
    loop {
        let byte = bytes.nextn(1);
        let char = byte_to_ascii(&byte).trim();
        if !char.is_empty() {
            height += char
        } else {
            break;
        }
    }
    let height: usize = height.parse().unwrap();

    // get the maximum u16 color value
    let mut maxval = String::from("");
    loop {
        let byte = bytes.nextn(1);
        let char = byte_to_ascii(&byte).trim();
        if !char.is_empty() {
            maxval += char
        } else {
            break;
        }
    }
    // we will parse as usize for convenience
    let maxval: usize = maxval.parse().unwrap();

    // for some reason, the maximum color value
    // cannot exceed 1. i do not understand the purpose
    assert!(
        0 < maxval,
        "Maximum color value is lower than 1, which is invalid in PPM formats."
    );
    assert!(
        maxval <= 65535,
        "Maximum color value exceeds the u16 limit of 65535, which is invalid in PPM formats."
    );

    // the image is height rows and width columns
    // a pixel is a triple of rgb in that order
    // it can either be 1 or 2 bytes.
    // convert into a row-major RGB image.
    let mut img = ImageRGB::new(width, height);
    let mut i = 0;

    if maxval < 256 {
        // 8 bit rgb channel
        // 1 bytes pixel
        while i < width * height {
            let rgb = bytes.nextn(3);
            img.push_rgb((rgb[0] as u16, rgb[1] as u16, rgb[2] as u16));
            i += 1;
        }
    } else {
        // 16 bit rgb channel
        // 2 bytes pixel, big endian
        while i < width * height {
            let rgb = bytes.nextn(6);
            let red = u16::from_be_bytes([rgb[0], rgb[1]]);
            let green = u16::from_be_bytes([rgb[2], rgb[3]]);
            let blue = u16::from_be_bytes([rgb[4], rgb[5]]);
            img.push_rgb((red, green, blue));
            i += 1;
        }
    }
    img
}

#[test]
fn test_ppm() {
    let _ = read_ppm("example/example.ppm");
}
