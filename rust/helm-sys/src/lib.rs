#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_char, CStr, CString};

pub const HELM_ERROR_PREFIX: &str = "ERROR:";

pub fn install_helm_release(
    release_name: &str,
    chart_name: &str,
    chart_version: &str,
    values_yaml: &str,
    namespace: &str,
    suppress_output: bool,
) -> String {
    let release_name = CString::new(release_name).unwrap();
    let chart_name = CString::new(chart_name).unwrap();
    let chart_version = CString::new(chart_version).unwrap();
    let values_yaml = CString::new(values_yaml).unwrap();
    let namespace = CString::new(namespace).unwrap();

    unsafe {
        let c = go_install_helm_release(
            release_name.as_ptr() as *mut c_char,
            chart_name.as_ptr() as *mut c_char,
            chart_version.as_ptr() as *mut c_char,
            values_yaml.as_ptr() as *mut c_char,
            namespace.as_ptr() as *mut c_char,
            suppress_output as u8,
        );

        cstr_ptr_to_string(c)
    }
}

pub fn uninstall_helm_release(
    release_name: &str,
    namespace: &str,
    suppress_output: bool,
) -> String {
    let release_name = CString::new(release_name).unwrap();
    let namespace = CString::new(namespace).unwrap();

    unsafe {
        let c = go_uninstall_helm_release(
            release_name.as_ptr() as *mut c_char,
            namespace.as_ptr() as *mut c_char,
            suppress_output as u8,
        );

        cstr_ptr_to_string(c)
    }
}

pub fn check_helm_release_exists(release_name: &str, namespace: &str) -> bool {
    let release_name = CString::new(release_name).unwrap();
    let namespace = CString::new(namespace).unwrap();

    unsafe {
        go_helm_release_exists(
            release_name.as_ptr() as *mut c_char,
            namespace.as_ptr() as *mut c_char,
        ) != 0
    }
}

pub fn list_helm_releases(namespace: &str) -> String {
    let namespace = CString::new(namespace).unwrap();

    unsafe {
        let c = go_helm_list_releases(namespace.as_ptr() as *mut c_char);
        cstr_ptr_to_string(c)
    }
}

pub fn add_helm_repository(repository_name: &str, repository_url: &str) -> String {
    let repository_name = CString::new(repository_name).unwrap();
    let repository_url = CString::new(repository_url).unwrap();

    unsafe {
        let c = go_add_helm_repo(
            repository_name.as_ptr() as *mut c_char,
            repository_url.as_ptr() as *mut c_char,
        );

        cstr_ptr_to_string(c)
    }
}

/// Checks if the result string is an error, and if so, returns the error message as a string.
pub fn to_helm_error(result: &str) -> Option<String> {
    if !result.is_empty() && result.starts_with(HELM_ERROR_PREFIX) {
        return Some(result.replace(HELM_ERROR_PREFIX, ""));
    }

    None
}

/// Converts a raw C string pointer into an owned Rust [`String`]. This functions
/// also makes sure, that the pointer (and underlying memory) of the Go string is
/// freed. The pointer **cannot** be used afterwards.
unsafe fn cstr_ptr_to_string(c: *mut c_char) -> String {
    let cstr = CStr::from_ptr(c);
    let s = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    free_go_string(cstr.as_ptr() as *mut c_char);

    s
}
