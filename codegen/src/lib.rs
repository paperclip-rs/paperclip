mod v3;

pub mod v3_03 {
    pub use super::v3::{OpenApiV3, PackageInfo};
}

#[cfg_attr(feature = "ramhorns-feat", macro_use)]
#[cfg(feature = "ramhorns-feat")]
extern crate log;
