pub mod spec {
    pub(crate) mod def;
    pub mod std {
        pub mod linux;
    }
}
pub(crate) mod lookup;
pub(crate) mod enums;
//pub(crate) mod uenv;

pub use crate::{
    enums::*,
    lookup::*,
    spec::def::*,
};

pub(crate) use std::{
    borrow::Cow,
    env,
    ffi::{OsStr,OsString},
    path::{self, PathBuf, Path},
    str::FromStr,
};