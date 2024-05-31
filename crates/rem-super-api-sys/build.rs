#![allow(unused_variables, unused_mut, dead_code)]
use clang::*;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    path::PathBuf,
};
#[macro_use]
extern crate lazy_static;
mod build_utils;

use crate::build_utils::{process_children, HandlerConfigs};
use build_utils::{create_handlers, HandlerMap};

lazy_static! {
    static ref THIRD_PARTY_PROJECT_DIR: PathBuf = env::current_dir().unwrap().join("thirdparty");
}

macro_rules! console_debug {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=thirdparty/wrapper.hpp");
    let generated_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/generated");
    if !generated_dir.exists() {
        fs::create_dir_all(&generated_dir).expect("Couldn't create generated directory");
    }
    create_mod_file(&generated_dir);

    generate_type(&generated_dir);
    clang_sys::load().expect("");
    let binding = Clang::new().unwrap();
    let index = Index::new(&binding, false, false);
    let wrapper_hpp_path = THIRD_PARTY_PROJECT_DIR.join("wrapper.hpp");
    let library_header_ast = index.parser(wrapper_hpp_path).parse().unwrap();
    let entity = library_header_ast.get_entity();
    let handlers = create_handlers();
    generate_api_wrapper(&entity, &handlers, &generated_dir);
    generate_spi_wrapper(&entity, &handlers, &generated_dir);
}

fn create_mod_file(generated_dir: &Path) {
    let mod_file_path = generated_dir.join("mod.rs");
    let mut mod_file = File::create(&mod_file_path).expect("Couldn't create mod.rs file");
    writeln!(mod_file, "pub mod bindings;").expect("Couldn't write to mod.rs file");
    writeln!(mod_file, "pub mod api_wrapper;").expect("Couldn't write to mod.rs file");
    writeln!(mod_file, "pub mod spi_wrapper;").expect("Couldn't write to mod.rs file");
}

/// 用 bindgen 生成与 C++ 代码兼容的 rust 的类型，生成的东西非常基本，还需要通过 unsafe 调用
fn generate_type(generated_dir: &Path) {
    // println!("cargo:rustc-link-lib=dylib=stdc++");

    let lib_path = THIRD_PARTY_PROJECT_DIR
        .join("rem_super_api")
        .join("v_current");
    assert!(
        lib_path.exists(),
        "Library path does not exist: {:?}",
        lib_path
    );

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
    println!("cargo:rustc-link-lib=thosttraderapi_se_local");

    let wrapper_header_path = THIRD_PARTY_PROJECT_DIR
        .join("wrapper.hpp")
        .to_str()
        .expect("Path to string conversion failed")
        .to_owned();

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header(wrapper_header_path)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .derive_default(true)
        .derive_debug(true)
        .vtable_generation(true)
        .generate_comments(false) // 不需形成doc ,默认true
        .layout_tests(false) //不需要test,默认true
        .generate_comments(false) //不需注释,默认true
        .derive_copy(true)
        .derive_hash(false) //不要实现hash
        .clang_arg(format!("-I{}", lib_path.join("include").display())) // Adjust include path as necessary
        .generate()
        .expect("Unable to generate bindings");

    let file_path = generated_dir.join("bindings.rs");
    bindings
        .write_to_file(&file_path)
        .expect("Couldn't write bindings!");
    // patch_generated_binding(&file_path);
}

/// 生成用于主动调用的 API 的 unsafe fn wrapper，以免每次在业务代码里调用 API 都要手动写
fn generate_api_wrapper(entity: &Entity, handlers: &HandlerMap, generated_dir: &Path) {
    let mut configs = HandlerConfigs::default();
    configs.record_flavor = build_utils::handlers::handle_record::RecordFlavor::API;
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("use crate::*;\n\n"));
    lines.extend(process_children(entity, handlers, &mut configs));
    let file_content = lines.join("");
    let file_path = generated_dir.join("api_wrapper.rs");
    let mut file = File::create(&file_path).expect("Unable to create api_wrapper.rs");
    file.write_all(file_content.as_bytes())
        .expect("Failed to write to api_wrapper.rs");
}

/// 生成用于被库调用的回调函数（在 C++ 生态里称为 SPI）的 unsafe fn wrapper，以免每次在业务代码里调用 API 都要手动写
fn generate_spi_wrapper(entity: &Entity, handlers: &HandlerMap, generated_dir: &Path) {
    let mut configs = HandlerConfigs::default();
    configs.record_flavor = build_utils::handlers::handle_record::RecordFlavor::SPI;
    let mut lines = process_children(entity, handlers, &mut configs);
    lines.push(format!("use crate::*;\n\n"));
    let file_content = lines.join("");
    let file_path = generated_dir.join("spi_wrapper.rs");
    let mut file = File::create(&file_path).expect("Unable to create spi_wrapper.rs");
    file.write_all(file_content.as_bytes())
        .expect("Failed to write to spi_wrapper.rs");

    // Debug 用，打印所有节点
    // library_header_ast.get_entity().visit_children(|e, _parent| {
    //     let name = e.get_display_name();
    //     let kind = e.get_type();
    //     if let Some(name) = name {
    //         if let Some(kind) = kind {
    //             console_debug!("name {name} ({:?})", kind);
    //         }
    //     }
    //     EntityVisitResult::Recurse
    // });
}

fn patch_generated_binding(file_path: &Path) {
    // Read the file content
    let mut content = fs::read_to_string(file_path).expect("Could not read file");

    // Add Arc and Mutex to the import statements
    // let import_statement = "use std::sync::{Arc, Mutex};\n";
    // content.insert_str(0, import_statement);

    // Replace the derive attribute for YDExchange
    content = content.replace(
        "#[derive(Copy, Clone)]",
        "#[derive(Debug, Copy, Clone)]",
    );

    // Write the modified content back to the file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Could not open file");
    file.write_all(content.as_bytes()).expect("Could not write to file");
}
