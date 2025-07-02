# An image processing library in Rust

Worked on this as a side project. Will generally contain functionalities to open and convert image formats, and do various transformations.


## Supported formats
**PBM (Portable Bitmap)** - Black and white pixels only, ASCII or binary format

Chore:
  - Current implementation mixes ASCII and binary (e.g, magic number in ASCII but pixels in binary). Need to separate two implementations.

---

- **PPM (Portable Pixmap)** - RGB color, uncompressed ASCII or binary data
Chore:
  - Current implementation mixes ASCII and binary (e.g, magic number in ASCII but pixels in binary). Need to separate two implementations.
