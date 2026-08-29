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
pub fn var<K: AsRef<str>>(key: K) -> Result<Cow<'static, str>, env::VarError> {
    match env::var(key.as_ref()) {
        Ok(v) => Ok(Cow::Owned(v)),
        Err(env::VarError::NotPresent) => match key {
            //crate::Usr::ASSET => Ok(Cow::Borrowed("foo")),
            _ => todo!()
        },
        Err(e @ env::VarError::NotUnicode(_)) => Err(e),
    }
}
