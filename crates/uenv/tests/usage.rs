use uenv;

#[test]
fn test_usage() {
    let sys_execute = uenv::Sys::Execute;
    let std_linux_spec = uenv::spec::std::linux::Sys;

    let actual = uenv::var("UENV_SYS_EXECUTE").expect("ok");
    assert_eq!("/usr/bin", actual);
    
    let actual = uenv::var(uenv::Sys::Execute).expect("ok");
    assert_eq!("/usr/bin", actual);
}