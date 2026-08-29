use crate::*;

#[inline]
pub fn uenv_spec() -> Cow<'static, str> {
    ::std::env::var(EnvSpec::ENV_SPEC)
        .map(|v| Cow::Owned(v))
        .unwrap_or_else(|_| Cow::Borrowed("xdg"))
}

#[inline]
pub fn uenv_var_enum(kind: Var) -> Result<Cow<'static, str>, env::VarError> {
    match env::var(kind.name()) {
        Ok(v) => Ok(Cow::Owned(v)),
        Err(env::VarError::NotPresent) => match kind {
            Var::Usr(Usr::Asset) => Ok(Cow::Borrowed("foo")),
            _ => todo!()
        },
        Err(e @ env::VarError::NotUnicode(_)) => Err(e),
    }
}

#[inline]
pub fn var<K>(key: K) -> Result<Cow<'static, str>, env::VarError>
where
    K: AsRef<str> + TryInto<Var>
{
    match env::var(key.as_ref()) {
        Ok(v) => Ok(Cow::Owned(v)),
        Err(env::VarError::NotPresent) => {
            if let Ok(key) = key.try_into() {
                match key {
                    Var::Usr(Usr::Asset) => Ok(Cow::Borrowed("foo")),
                    _ => todo!()
                }
            } else {
                Err(env::VarError::NotPresent)
            }
        },
        Err(e @ env::VarError::NotUnicode(_)) => Err(e),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Val {
    Str(Cow<'static, str>),
    Dir(Cow<'static, Path>),
    DirList(Vec<PathBuf>)
}

#[inline]
pub fn val<K>(key: K) -> Result<Val, env::VarError>
where
    K: AsRef<str> + TryInto<Var, Error = K>
{
    match key.try_into() {
        Ok(key) => val_key(key),
        Err(key) => val_str(key.as_ref()),
    }
}

fn val_key(key: Var) -> Result<Val, env::VarError> {
    todo!()
}

fn val_str(key: &str) -> Result<Val, env::VarError> {
    todo!()
}

#[inline]
pub fn value<K>(key: K) -> Result<K::ValType, env::VarError>
where
    K: Variant
{
    todo!()
}
