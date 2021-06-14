pub enum Orientation {
    Unknown = 0,
    Default = 1,
    Rotate90 = 2,
    Rotate180 = 3,
    Rotate270 = 4,
}

cfg_if! {
    if #[cfg(quartz)] {
        mod quartz;
        pub use self::quartz::*;
    } else if #[cfg(x11)] {
        mod x11;
        pub use self::x11::*;
    } else if #[cfg(dxgi)] {
        mod dxgi;
        pub use self::dxgi::*;
    } else {
        //TODO: Fallback implementation.
    }
}
