use crate::*;

pub struct SpecDef {
    /// the path to expect it at
    pub pathed: Pathed,
    /// if not empty: check path existence and attempt to fallback to another def,
    /// in order of preference
    pub exist_fallbacks: &'static [&'static SpecDef],
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EnvSpec {
    Xdg,
    DotSys,
}

impl EnvSpec {
    pub const ENV_SPEC: &'static str =  "ENV_SPEC";
    pub const XDG: &'static str = "xdg";
    pub const DOTSYS: &'static str = "dotsys";
    
    pub const fn env_name() -> &'static str { Self::ENV_SPEC }
    pub const fn env_val(&self) -> &'static str {
        match self {
            Self::Xdg => Self::XDG,
            Self::DotSys => Self::DOTSYS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TargetOs {
    Other,
    Linux,
    MacOs,
    Windows,
}

impl TargetOs {
    pub const fn current() -> Self {
        cfg_select! {
            target_os = "linux" => Self::Windows,
            target_os = "macos" => Self::MacOs,
            target_os = "windows" => Self::Windows,
            _ => Self::Other,
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pth {
    Root,
    Home,
    Username,
    UnixId,
    WindowsSid,
    Prefix(path::Prefix<'static>),
    Str(&'static str),
}

enum PthPart<'a> {
    Component(path::Component<'a>),
    Owned(OsString),
    Components(Vec<path::Component<'a>>),
}

pub struct Pathed(pub &'static [Pth]);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PthParams {
    /// perform fallback checks
    pub username: Option<String>,
    pub user_unix_id: Option<u32>,
    pub home_dir: Option<PathBuf>,
}
