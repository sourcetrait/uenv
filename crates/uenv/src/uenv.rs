use crate::*;

impl Pth {
    pub fn parts<'a>(&'static self, params: &'a PthParams, mut s: OsString) -> Result<PthPart<'a>, ()> {
        match self {
            Self::Root => Ok(PthPart::Component(path::Component::RootDir)),
            Self::Home => match params.home_dir.as_ref() {
                Some(p) => Ok(PthPart::Components(p.components().collect())),
                None => Err(()),
            },
            Self::Username => match params.username.as_ref() {
                Some(s) => Ok(PthPart::Component(path::Component::Normal(OsStr::new(s)))),
                None => Err(()),
            },
            Self::UnixId => match params.user_unix_id {
                Some(n) => Ok(PthPart::Owned(n.to_string().into())),
                None => Err(()),
            },
            Self::Prefix(x) => Ok(PthPart::Component(x.into())),
            _ => todo!()
        }
    }
}

impl Pathed {
    pub fn expand<'a>(&'static self, params: &'a PthParams) -> PathBuf {
        let mut parts: Vec<path::Component> = vec![];
        for pth in self.0 {
            parts.extend(pth.parts(&params));
        }

        PathBuf::from_iter(parts)
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
        match var(key) {
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

