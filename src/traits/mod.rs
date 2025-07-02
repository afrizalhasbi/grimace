macro_rules! expose {
    ($m:ident) => {
        pub mod $m; // load `$m.rs`
        pub use $m::*; // re‑export everything
    };
}

expose!(image);
