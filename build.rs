use std::env;
use std::fs::{self, copy};
use std::path::Path;
use std::process::Command;

fn main() {
    // Version of the LocalCTP to use
    let version = "6.7.0";

    // Directory of the LocalCTP submodule and output library
    let local_ctp_path = "./LocalCTP";
    let build_script_path = format!("{}/buildLinux.sh", local_ctp_path);
    let lib_src_path = format!("{}/bin/linux/libthosttraderapi_se_v{}.so", local_ctp_path, version);
    let ctp_files_src_dir = format!("{}/LocalCTP/ctp_file/{}", local_ctp_path, version);

    // Target directory for the system-specific crate
    let localctp_sys_path = "./crates/localctp-sys/thirdparty/LocalCTP/v_current";
    let lib_dst_path = format!("{}/libthosttraderapi_se.so", localctp_sys_path);

    // Build the LocalCTP project
    println!("Running build script for LocalCTP...");
    let status = Command::new("sh")
        .arg(build_script_path)
        .status()
        .expect("Failed to run the build script");

    if !status.success() {
        panic!("Failed to build LocalCTP: {}", status);
    }

    // Ensure the destination directory exists
    fs::create_dir_all(localctp_sys_path).unwrap();

    // Move the shared library
    println!("Moving shared library to {}", localctp_sys_path);
    fs::rename(&lib_src_path, &lib_dst_path).expect("Failed to move the shared library");

    // Copy other necessary CTP files
    println!("Copying CTP files to {}", localctp_sys_path);
    let file_names = [
        "ThostFtdcMdApi.h", "ThostFtdcUserApiDataType.h", "error.dtd",
        "libthostmduserapi_se.so", "thostmduserapi_se.dll", "thosttraderapi_se.dll",
        "ThostFtdcTraderApi.h", "ThostFtdcUserApiStruct.h", "error.xml",
        "libthosttraderapi_se.so", "thostmduserapi_se.lib", "thosttraderapi_se.lib",
    ];

    for file_name in file_names.iter() {
        let src_file = Path::new(&ctp_files_src_dir).join(file_name);
        let dst_file = Path::new(localctp_sys_path).join(file_name);
        if src_file.exists() {
            copy(&src_file, &dst_file).expect(&format!("Failed to copy {}", file_name));
        }
    }

    println!("Build script completed successfully.");
}
