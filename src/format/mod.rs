macro_rules! expose {
    ($m:ident) => {
        pub mod $m; // load `$m.rs`
        pub use $m::*; // re‑export everything
    };
}

expose!(bmp);
expose!(jpg);
expose!(png);
expose!(pbm);
// expose!(pgm);
expose!(ppm);
