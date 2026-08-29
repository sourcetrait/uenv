use crate::*;

/// System Directories: Distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Sys {
    /// `/tmp`
    Temporary,
    /// `/usr/bin`
    Execute,
    /// `/usr/lib`
    Library,
    /// `/etc`
    Configuration,
    /// `/usr/share`
    Asset,
    /// `/opt`
    Package,
    /// `/var`
    Variable,
    /// `/usr/sbin`
    SuperExecute,
}

impl Sys {
    pub const TEMPORARY: &'static str = "UENV_SYS_TEMPORARY";
    pub const EXECUTE: &'static str = "SDIR_EXECUTE";
    pub const LIBRARY: &'static str = "SDIR_LIBRARY";
    pub const CONFIGURATION: &'static str = "UENV_SYS_CONFIGURATION";
    pub const ASSET: &'static str = "SDIR_ASSET";
    pub const PACKAGE: &'static str = "SDIR_PACKAGE";
    pub const VARIABLE: &'static str = "SDIR_VARIABLE";
    pub const SUPER_EXECUTE: &'static str = "SDIR_SUPER_EXECUTE";
    
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Temporary => Self::TEMPORARY,
            Self::Execute => Self::EXECUTE,
            Self::Library => Self::LIBRARY,
            Self::Configuration => Self::CONFIGURATION,
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

impl AsRef<str> for Sys {
    fn as_ref(&self) -> &str { self.name() }
}

impl<'a> TryFrom<&'a str> for Var {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Err(value) //todo
    }
}

impl TryFrom<Sys> for Var {
    type Error = Sys;

    fn try_from(value: Sys) -> Result<Self, Self::Error> {
        Err(value) // todo
    }
}

/// User Directories: Internal software operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Usr {
    /// `/home/ulric/.cache`, `/home/ulric/.sys/cache`
    Cache,
    /// `/home/ulric/.config`, `/home/ulric/.config`
    Config,
    /// `/home/ulric/.config`, `/home/ulric/.config` (windows: AppData/Roaming)
    ConfigSync,
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
    Configuration,
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
    ShellConfig,
    /// `/home/ulric/.sys/mess`
    Mess,
    /// `/run/user/1000`
    Runtime,
    /// `/dev/shm/ulric`
    Transitive,
}

impl Usr {
    pub const CACHE: &'static str = "UENV_USR_CACHE";
    pub const CONFIG: &'static str = "UENV_USR_CONFIG";
    pub const CONFIG_SYNC: &'static str = "UENV_USR_CONFIG_SYNC";
    pub const DATA: &'static str = "UENV_USR_DATA";
    pub const STATE: &'static str = "UENV_USR_STATE";
    pub const TEMPORARY: &'static str = "UENV_USR_TEMPORARY";
    pub const EXECUTE: &'static str = "UENV_USR_EXECUTE";
    pub const LIBRARY: &'static str = "UENV_USR_LIBRARY";
    pub const CONFIGURATION: &'static str = "UENV_USR_CONFIGURATION";
    pub const ASSET: &'static str = "UENV_USR_ASSET";
    pub const PACKAGE: &'static str = "UENV_USR_PACKAGE";
    pub const VARIABLE: &'static str = "UENV_USR_VARIABLE";
    pub const SECRET_CACHE: &'static str = "UENV_USR_SECRET_CACHE";
    pub const SECRET_CONFIG: &'static str = "UENV_USR_SECRET_CONFIG";
    pub const SECRET_DATA: &'static str = "UENV_USR_SECRET_DATA";
    pub const SECRET_KEY: &'static str = "UENV_USR_SECRET_KEY";
    pub const SHELL_CONFIG: &'static str = "UENV_USR_SHELL_CONFIG";
    pub const MESS: &'static str = "UENV_USR_MESS";
    pub const TRANSITIVE: &'static str = "UENV_USR_TRANSITIVE";
    pub const RUNTIME: &'static str = "UENV_USR_RUNTIME";
}
    
impl Usr {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Cache => Self::CACHE,
            Self::Config => Self::CONFIG,
            Self::ConfigSync => Self::CONFIG_SYNC,
            Self::Data => Self::DATA,
            Self::State => Self::STATE,
            Self::Temporary => Self::TEMPORARY,
            Self::Execute => Self::EXECUTE,
            Self::Library => Self::LIBRARY,
            Self::Configuration => Self::CONFIGURATION,
            Self::Asset => Self::ASSET,
            Self::Package => Self::PACKAGE,
            Self::Variable => Self::VARIABLE,
            Self::SecretCache => Self::SECRET_CACHE,
            Self::SecretConfig => Self::SECRET_CONFIG,
            Self::SecretData => Self::SECRET_DATA,
            Self::SecretKey => Self::SECRET_KEY,
            Self::ShellConfig => Self::SHELL_CONFIG,
            Self::Mess => Self::MESS,
            Self::Transitive => Self::TRANSITIVE,
            Self::Runtime => Self::RUNTIME,
        }
    }
    
    pub const fn from_name(s: &str) -> Option<Self> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum My {
    Documents,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Local {
    /// `/usr/local/bin`
    Execute,
    /// `/usr/local/lib`
    Library,
    /// `/usr/local/etc`
    Setting,
    /// `/usr/local/share`
    Asset,
    /// `/opt`
    Package,
    /// `/var/local`
    Variable,
    /// `/usr/local/sbin`
    SuperExecute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Var {
    Sys(Sys),
    Usr(Usr),
}

impl Var {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Sys(sdir) => sdir.name(),
            Self::Usr(udir) => udir.name(),
        }
    }

    pub const fn from_name(s: &str) -> Option<Self> {
        if let Some(udir) = Usr::from_name(s) {
            Some(Self::Usr(udir))
        } else if let Some(sdir) = Sys::from_name(s) {
            Some(Self::Sys(sdir))
        } else {
            None
        }
    }
}

impl FromStr for Var {
    type Err = &'static str;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s)
            .ok_or_else(|| "Unknown uenv variable name")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecVal {
    pub val: Val,
    pub fell: Option<Var>,
    pub standards: Vec<Standard>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Standard {
    Owner(User),
    OwnerGroup(Group),
    Owners(User, Group),
    Permissions(Permissions),
    Capabilities(Vec<Capability>),
}