pub mod spec {
    pub(crate) mod def;
    pub mod std {
        pub mod linux;
    }
}
pub(crate) mod os;
pub(crate) mod lookup;
pub(crate) mod enums;
pub mod sys;
pub mod prelude;
//pub(crate) mod uenv;

pub(crate) use crate::{
    prelude::*,
    os::*,
};

pub use crate::{
    enums::*,
    lookup::*,
    spec::def::*,
};

pub(crate) use std::{
    borrow::Cow,
    env,
    mem,
    ffi::{OsStr,OsString},
    path::{self, PathBuf, Path},
    str::FromStr,
};