use crate::traits::ImageRGB;
use crate::utils::{ByteIterator, byte_to_ascii, iter_bits, read_file};

pub fn read_pbm(file: &str) -> ImageRGB {
    let mut bytes = ByteIterator::from_file(file);
    let header = bytes.nextn(3);
    let header_ascii = byte_to_ascii(&header).trim_ascii_end();

    // ASCII magic number is P1
    // The binary variation is P4
    assert_eq!(
        header_ascii, "P1",
        "Invalid magic number in the header. Expected P1, got {}",
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

    // now we get the image. its a 1 bit raster
    // width is byte-packed, e.g., divisible by 8 bits
    // let height = 8; let width = 2;
    // 0 0 0 0 0 1 1 1
    // 0 1 1 1 1 0 0 0
    // convert into a row-major RGB image
    let mut img = ImageRGB::new(width, height);
    let mut i = 0;
    while i < width * height {
        let byte = bytes.next();
        for bit in iter_bits(byte) {
            // in pbm 0 represents white
            let bit: u16 = if bit == 0 { 65535 } else { 0 };
            img.push_rgb((bit, bit, bit));
        }
        i += 1
    }
    img
}

#[test]
fn test_pbm() {
    let _ = read_pbm("example/example.pbm");
}
