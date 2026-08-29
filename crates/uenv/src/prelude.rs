use crate::*;

pub trait Variant {
    const VARIANT: Self::EnumType;
    type EnumType: Sized;
    type ValType: Sized;
}

