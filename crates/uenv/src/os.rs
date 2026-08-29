use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum User {
    ContextName,
    Name(UserName),
    Unix(UnixUser),
    Windows(WindowsUser),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnixUser {
    /// root
    Zero,
    ContextId,
    Id(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Group {
    Name(GroupName),
    Unix(UnixGroup),
    MacOs(MacOsGroup),
    Windows(WindowsGroup),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnixGroup {
    /// root, wheel
    Zero,
    ContextPrimary,
    Id(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacOsGroup {
    Staff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsUser {
    ContextSid,
    WindowsSid(Cow<'static, str>),
    WellKnown(WindowsWellKnownUser),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsWellKnownUser {
    Administrator,
    Guest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsGroup {
    ContextSid,
    Builtin(WindowsBuiltinGroup),
    WellKnown(WindowsWellKnownGroup),
    WellKnownDomain(WindowsWellKnownDomainGroup),
    Sid(Cow<'static, str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsBuiltinGroup {
    Administrators,
    Users,
    Guests,
    PowerUsers,
    RemoteDesktopUsers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsWellKnownDomainGroup {
    DomainAdmins,
    DomainUsers,
    DomainGuests,
    DomainComputers,
    DomainControllers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsWellKnownGroup {
    Everyone,
    AuthenticatedUsers,
    System,
    LocalService,
    NetworkService,
    CreatorOwner,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(pub Cow<'static, str>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupName(pub Cow<'static, str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Permit {
    None = 0o0,
    Execute = 0o1,
    Write = 0o2,
    Read = 0o4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Permission(pub u8);
impl Permission {
    pub const fn rwx(read: u8, write: u8, execute: u8) -> Self {
        Self(read | write | execute)
    }
    
    pub const fn permit(read: Permit, write: Permit, execute: Permit) -> Self {
        Self(read as u8 | write as u8 | execute as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Permissions(pub u32);
impl Permissions {
    pub const fn ugo(user: u8, group: u8, other: u8) -> Self {
        Self((user as u32) << 6 | (group as u32) << 3 | other as u32)
    }
    
    pub const fn permit(user: Permission, group: Permission, other: Permission) -> Self {
        Self((user.0 as u32) << 6 | (group.0 as u32) << 3 | other.0 as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Unix(UnixCapability),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixCapability {
    Sockets,
}
