#![allow(non_snake_case)]

use uenv;
use std::{
    borrow::Cow,
    path::Path,
};

const OK: &'static str = "ok";

#[test]
fn test_equivalence() {
    const VAR: uenv::Var = uenv::Var::Sys(uenv::Sys::Execute);
    const ENUM: uenv::Sys = uenv::Sys::Execute;
    const STRUCT: uenv::sys::Execute = uenv::sys::Execute;
    const VAR_STR: &'static str = "UENV_SYS_EXECUTE";
    const VAL_STR: &'static str = "/usr/bin";
    let VAL: uenv::Val = uenv::Val::Dir(Cow::Borrowed(Path::new(VAL_STR)));
    let VALUE: Cow<'static, Path> = Cow::Borrowed(Path::new(VAL_STR));
    
    let actual = uenv::var(VAR_STR).expect(OK);
    assert_eq!(VAL_STR, actual);
    
    let actual = uenv::var(ENUM).expect(OK);
    assert_eq!(VAL_STR, actual);

    let actual = uenv::val(VAR_STR).expect(OK);
    assert_eq!(VAL, actual);
    
    let actual = uenv::val(ENUM).expect(OK);
    assert_eq!(VAL, actual);
    
    let actual = uenv::val(STRUCT).expect(OK);
    assert_eq!(VAL, actual);

    let actual = uenv::value(STRUCT).expect(OK);
    assert_eq!(VALUE, actual);
    
    let actual = uenv::specific(uenv::sys::Execute).expect(OK);
    assert_eq!(Cow::Borrowed(Path::new(VAL_STR)), actual);
}