use std::ffi::CStr;
use localctp_sys::{get_api_version, create_local_api_and_spi};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref VERSION: String = "LocalCTP V1.0.0 By QiuShui(Aura) QQ1005018695".to_string();
}

fn get_api_version1() -> Option<String> {
    unsafe {
        let version_c_str = get_api_version(); // Adjusted to use the bindings module
        if version_c_str.is_null() {
            None
        } else {
            Some(CStr::from_ptr(version_c_str).to_string_lossy().into_owned())
        }
    }
}

#[test]
fn test_get_api_version() {
    let version = get_api_version1().expect("Failed to get API version");
    assert!(!version.is_empty(), "API version should not be empty.");
    assert_eq!(version, *VERSION, "API version should be updated.");
}
