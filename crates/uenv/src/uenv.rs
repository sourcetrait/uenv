use crate::*;

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

/// System Directories: Distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SDir {
    /// `/tmp`
    Temporary,
    /// `/usr/bin`
    Execute,
    /// `/usr/lib`
    Library,
    /// `/etc`
    Setting,
    /// `/usr/share`
    Asset,
    /// `/opt`
    Package,
    /// `/var`
    Variable,
    /// `/usr/sbin`
    SuperExecute,
}

impl SDir {
    pub const TEMPORARY: &'static str = "SDIR_TEMPORARY";
    pub const EXECUTE: &'static str = "SDIR_EXECUTE";
    pub const LIBRARY: &'static str = "SDIR_LIBRARY";
    pub const SETTING: &'static str = "SDIR_SETTING";
    pub const ASSET: &'static str = "SDIR_ASSET";
    pub const PACKAGE: &'static str = "SDIR_PACKAGE";
    pub const VARIABLE: &'static str = "SDIR_VARIABLE";
    pub const SUPER_EXECUTE: &'static str = "SDIR_SUPER_EXECUTE";
    
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Temporary => Self::TEMPORARY,
            Self::Execute => Self::EXECUTE,
            Self::Library => Self::LIBRARY,
            Self::Setting => Self::SETTING,
            Self::Asset => Self::ASSET,
            Self::Package => Self::PACKAGE,
            Self::Variable => Self::VARIABLE,
            Self::SuperExecute => Self::SUPER_EXECUTE,
        }
    }
    
    pub const fn from_name(s: &str) -> Option<Self> {
        None
    }
}

/// User Directories: Internal software operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UDir {
    /// `/home/ulric/.cache`, `/home/ulric/.sys/cache`
    Cache,
    /// `/home/ulric/.config`, `/home/ulric/.config`
    Config,
    /// `/home/ulric/.local/share`, `/home/ulric/.sys/data`
    Data,
    /// `/home/ulric/.local/state`, `/home/ulric/.sys/state`
    State,
    /// `/tmp`, `/home/ulric/tmp`
    Temporary,
    /// `/home/ulric/.local/bin`, `/home/ulric/.sys/install/bin`
    Execute,
    /// `/home/ulric/.sys/install/lib`
    Library,
    /// `/home/ulric/.sys/install/etc`
    Setting,
    /// `/home/ulric/.sys/install/share`
    Asset,
    /// `/home/ulric/.sys/install/opt`
    Package,
    /// `/home/ulric/.sys/install/var`
    Variable,
    /// `/home/ulric/.sys/secret/cache`
    SecretCache,
    /// `/home/ulric/.sys/secret/config`
    SecretConfig,
    /// `/home/ulric/.sys/secret/data`
    SecretData,
    /// `/home/ulric/.sys/secret/key`
    SecretKey,
    /// `/home/ulric/.ssh`
    ShellKey,
    /// `/home/ulric/.sys/mess`
    Mess,
    /// `/dev/shm/ulric`
    Memory,
    /// `/run/user/1000`
    Transitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pth {
    Root,
    Home,
    Username,
    UnixId,
    WindowsSid,
    Prefix(&'static str),
    N(&'static str),
}
impl Pth {
    pub fn component<'a>(&'static self, params: &'a PthParams) -> Vec<path::Component<'a>> {
        match self {
            Self::Root => vec![path::Component::RootDir],
            Self::Home => {
                if let Some(home_dir) = params.home_dir.as_ref() {
                    home_dir.components().collect()
                } else {
                    vec![path::Component::Normal(OsStr::new("$HOME"))]
                }
            },
            _ => todo!()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PthParams {
    /// perform fallback checks
    pub username: Option<String>,
    pub user_unix_id: Option<u32>,
    pub home_dir: Option<PathBuf>,
}

pub struct Pathed(pub &'static [Pth]);
impl Pathed {
    pub fn expand<'a>(&'static self, params: &'a PthParams) -> PathBuf {
        let mut parts: Vec<path::Component> = vec![];
        for pth in self.0 {
            parts.extend(pth.component(&params));
        }

        PathBuf::from_iter(parts)
    }
}

impl UDir {
    pub const CACHE: &'static str = "UDIR_CACHE";
    pub const CONFIG: &'static str = "UDIR_CONFIG";
    pub const DATA: &'static str = "UDIR_DATA";
    pub const STATE: &'static str = "UDIR_STATE";
    pub const TEMPORARY: &'static str = "UDIR_TEMPORARY";
    pub const EXECUTE: &'static str = "UDIR_EXECUTE";
    pub const LIBRARY: &'static str = "UDIR_LIBRARY";
    pub const SETTING: &'static str = "UDIR_SETTING";
    pub const ASSET: &'static str = "UDIR_ASSET";
    pub const PACKAGE: &'static str = "UDIR_PACKAGE";
    pub const VARIABLE: &'static str = "UDIR_VARIABLE";
    pub const SECRET_CACHE: &'static str = "UDIR_SECRET_CACHE";
    pub const SECRET_CONFIG: &'static str = "UDIR_SECRET_CONFIG";
    pub const SECRET_DATA: &'static str = "UDIR_SECRET_DATA";
    pub const SECRET_KEY: &'static str = "UDIR_SECRET_KEY";
    pub const SHELL_KEY: &'static str = "UDIR_SHELL_KEY";
    pub const MESS: &'static str = "UDIR_MESS";
    pub const MEMORY: &'static str = "UDIR_MEMORY";
    pub const TRANSITIVE: &'static str = "UDIR_TRANSITIVE";
}
    
impl UDir {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Cache => Self::CACHE,
            Self::Config => Self::CONFIG,
            Self::Data => Self::DATA,
            Self::State => Self::STATE,
            Self::Temporary => Self::TEMPORARY,
            Self::Execute => Self::EXECUTE,
            Self::Library => Self::LIBRARY,
            Self::Setting => Self::SETTING,
            Self::Asset => Self::ASSET,
            Self::Package => Self::PACKAGE,
            Self::Variable => Self::VARIABLE,
            Self::SecretCache => Self::SECRET_CACHE,
            Self::SecretConfig => Self::SECRET_CONFIG,
            Self::SecretData => Self::SECRET_DATA,
            Self::SecretKey => Self::SECRET_KEY,
            Self::ShellKey => Self::SHELL_KEY,
            Self::Mess => Self::MESS,
            Self::Memory => Self::MEMORY,
            Self::Transitive => Self::TRANSITIVE,
        }
    }
    
    pub const fn from_name(s: &str) -> Option<Self> {
        None
    }
}

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MDir {
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UEnv {
    SDir(SDir),
    UDir(UDir),
}

impl UEnv {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::SDir(sdir) => sdir.name(),
            Self::UDir(udir) => udir.name(),
        }
    }

    pub const fn from_name(s: &str) -> Option<Self> {
        if let Some(udir) = UDir::from_name(s) {
            Some(Self::UDir(udir))
        } else if let Some(sdir) = SDir::from_name(s) {
            Some(Self::SDir(sdir))
        } else {
            None
        }
    }
}

impl FromStr for UEnv {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s)
            .ok_or_else(|| "Unknown uenv variable name")
    }
}

#[inline]
pub fn uenv_spec() -> Cow<'static, str> {
    ::std::env::var(EnvSpec::ENV_SPEC)
        .map(|v| Cow::Owned(v))
        .unwrap_or_else(|_| Cow::Borrowed("xdg"))
}

#[inline]
pub fn uenv_var_enum(kind: UEnv) -> Result<Cow<'static, str>, env::VarError> {
    match env::var(kind.name()) {
        Ok(v) => Ok(Cow::Owned(v)),
        Err(env::VarError::NotPresent) => match kind {
            UEnv::UDir(UDir::Asset) => Ok(Cow::Borrowed("foo")),
            _ => todo!()
        },
        Err(e @ env::VarError::NotUnicode(_)) => Err(e),
    }
}

#[inline]
pub fn uenv_var(key: &str) -> Result<Cow<'static, str>, env::VarError> {
    match env::var(key) {
        Ok(v) => Ok(Cow::Owned(v)),
        Err(env::VarError::NotPresent) => match key {
            UDir::ASSET => Ok(Cow::Borrowed("foo")),
            _ => todo!()
        },
        Err(e @ env::VarError::NotUnicode(_)) => Err(e),
    }
}

pub fn uenv_expand(s: &str) -> Result<Cow<'_, str>, String> {
    #[inline]
    pub fn lookup_home_dir() -> Option<String> {
        env::home_dir()
            .map(|v| v.into_string().expect("UTF8"))
    }
    
    #[inline]
    pub fn lookup_uenv_var(key: &str) -> Result<Option<Cow<'static, str>>, String> {
        match uenv_var(key) {
            Ok(v) => Ok(Some(v)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(name)) => Err(name.into_string().expect("UTF8")),
        }
    }
    
    match shellexpand::full_with_context(s, lookup_home_dir, lookup_uenv_var) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.var_name),
    }
}

pub struct SpecDef {
    /// the path to expect it at
    pub pathed: Pathed,
    /// if not empty: check path existence and attempt to fallback to another def,
    /// in order of preference
    pub exist_fallbacks: &'static [&'static SpecDef],
}

impl SpecDef {
    pub fn expand(&'static self, params: &PthParams, fallback: bool) -> Option<PathBuf> {
        let path = self.pathed.expand(params);
        if !fallback {
            Some(path)
        } else if self.exist_fallbacks.is_empty() {
            Some(path)
        } else if path.exists() {
            Some(path)
        } else {
            for fallback in self.exist_fallbacks {
                let path = fallback.expand(params, true);
                if path.is_some() {
                    return path;
                }
            }

            None
        }
    }
}

pub struct SpecLinuxStandardSdir;
impl SpecLinuxStandardSdir {
    const TEMPORARY: SpecDef = SpecDef {
        pathed: Pathed(&[Pth::Root, Pth::N("tmp")]),
        exist_fallbacks: &[],
    };
}

pub struct SpecLinuxStandardUdir;
impl SpecLinuxStandardUdir {
    const CACHE: SpecDef = SpecDef {
        pathed: Pathed(&[Pth::Home, Pth::N(".cache")]),
        exist_fallbacks: &[],
    };
    const CONFIG: Pathed = Pathed(&[Pth::Home, Pth::N(".config")]);
    const DATA: Pathed = Pathed(&[Pth::Home, Pth::N(".local"), Pth::N("share")]);
    const STATE: Pathed = Pathed(&[Pth::Home, Pth::N(".local"), Pth::N("state")]);
    const TRANSITIVE: SpecDef = SpecDef {
        pathed: Pathed(&[Pth::Root, Pth::N("run"), Pth::N("user"), Pth::UnixId]),
        exist_fallbacks: &[&SpecLinuxStandardSdir::TEMPORARY],
    };

    pub fn expand(udir: UDir, params: &PthParams) -> PathBuf {
        match udir {
            //UDir::Cache => Self::CACHE.expand(params),
            //UDir::Config => Self::CONFIG.expand(params),
            //UDir::Data => Self::DATA.expand(params),
            //UDir::State => Self::STATE.expand(params),
            //UDir::Transitive => Self::TRANSITIVE.expand(params),
            //UDir::Temporary => Self::TRANSITIVE.expand(params, true),
            _ => todo!(),
        }
    }
}