use std::path::PathBuf;

use wdk_build::{get_km_dir, DirectoryType};

fn generate_base() {
    println!("cargo:rerun-if-changed=wrapper/wrapper.h");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();
    let out_path = PathBuf::from(
        std::env::var_os("OUT_DIR").expect("the environment variable OUT_DIR is undefined"),
    );

    bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .use_core()
        .derive_debug(false)
        .layout_tests(false)
        .ctypes_prefix("cty")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ignore_functions()
        .generate()
        .unwrap()
        .write_to_file(out_path.join("base.rs"))
        .unwrap();
}

#[cfg(feature = "ntoskrnl")]
fn generate_ntoskrnl() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rustc-link-lib=ntoskrnl");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();
    let out_path = PathBuf::from(
        std::env::var_os("OUT_DIR").expect("the environment variable OUT_DIR is undefined"),
    );

    bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .use_core()
        .derive_debug(false)
        .layout_tests(false)
        .ctypes_prefix("cty")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type(".*")
        .allowlist_function(".*")
        .allowlist_recursively(false)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("ntoskrnl.rs"))
        .unwrap();

    //cc::Build::new()
    //    .flag("/kernel")
    //    .include(include_dir)
    //    .file("src/wrapper.c")
    //    .compile("wrapper_ntoskrnl");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    generate_base();
    generate_ntoskrnl();
}
