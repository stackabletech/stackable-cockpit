use std::{marker::PhantomData, os::raw::c_char};

#[repr(C)]
pub struct GoString<'a> {
    p: *const u8,
    n: i64,
    _lifetime: PhantomData<&'a str>,
}

impl<'a> From<&'a str> for GoString<'a> {
    fn from(str: &'a str) -> Self {
        GoString {
            p: str.as_ptr(),
            n: str.len() as i64,
            _lifetime: PhantomData,
        }
    }
}

extern "C" {
    pub fn go_install_helm_release(
        release_name: GoString,
        chart_name: GoString,
        chart_version: GoString,
        values_yaml: GoString,
        namespace: GoString,
        suppress_output: bool,
    ) -> *const c_char;
    pub fn go_uninstall_helm_release(
        release_name: GoString,
        namespace: GoString,
        suppress_output: bool,
    ) -> *const c_char;
    pub fn go_helm_release_exists(release_name: GoString, namespace: GoString) -> bool;
    pub fn go_helm_list_releases(namespace: GoString) -> *const c_char;
    pub fn go_add_helm_repo(name: GoString, url: GoString) -> *const c_char;
}
