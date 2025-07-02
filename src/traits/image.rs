pub trait Image {
    fn as_rgb() {}
    fn as_hsl() {}
    fn save() {}
}

pub struct ImageRGB {
    width: usize,
    height: usize,
    // 16 bit color channels by default
    // 8 bit channels are cast to u16
    red: Vec<u16>,
    green: Vec<u16>,
    blue: Vec<u16>,
}
impl ImageRGB {
    // size is row length
    pub fn new(width: usize, height: usize) -> ImageRGB {
        let size = width * height;
        ImageRGB {
            width: width,
            height: height,
            red: Vec::with_capacity(size),
            green: Vec::with_capacity(size),
            blue: Vec::with_capacity(size),
        }
    }
    pub fn push_rgb(&mut self, rgb: (u16, u16, u16)) {
        self.red.push(rgb.0);
        self.green.push(rgb.1);
        self.blue.push(rgb.2);
    }
}

pub struct ImageHSL {}
