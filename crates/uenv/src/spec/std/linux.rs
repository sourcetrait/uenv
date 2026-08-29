use crate::*;

pub struct Sys;
impl Sys {
    const TEMPORARY: SpecDef = SpecDef {
        pathed: Pathed(&[Pth::Root, Pth::Str("tmp")]),
        exist_fallbacks: &[],
    };
}

pub struct Usr;
impl Usr {
    const CACHE: SpecDef = SpecDef {
        pathed: Pathed(&[Pth::Home, Pth::Str(".cache")]),
        exist_fallbacks: &[],
    };
    const CONFIG: Pathed = Pathed(&[Pth::Home, Pth::Str(".config")]);
    const DATA: Pathed = Pathed(&[Pth::Home, Pth::Str(".local"), Pth::Str("share")]);
    const STATE: Pathed = Pathed(&[Pth::Home, Pth::Str(".local"), Pth::Str("state")]);
    const TRANSITIVE: SpecDef = SpecDef {
        pathed: Pathed(&[Pth::Root, Pth::Str("run"), Pth::Str("user"), Pth::UnixId]),
        exist_fallbacks: &[&Sys::TEMPORARY],
    };

    pub fn expand(udir: Usr, params: &PthParams) -> PathBuf {
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