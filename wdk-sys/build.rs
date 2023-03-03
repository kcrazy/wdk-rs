use bindgen::Abi;
use wdk_build::{get_km_dir, DirectoryType};

fn generate_base() {
    println!("cargo:rerun-if-changed=wrapper/wrapper.h");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();

    let buf = bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .use_core()
        .override_abi(Abi::Stdcall, r"\w*")
        .derive_debug(false)
        .layout_tests(false)
        .ctypes_prefix("cty")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ignore_functions()
        .generate()
        .unwrap()
        .to_string();

    let buf = buf.replace("extern \"C\"", "extern \"stdcall\"");
    std::fs::write("src/bind/base.rs", buf).expect("Fail to write converted bindings!");
}

#[cfg(feature = "ntoskrnl")]
fn generate_ntoskrnl() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rustc-link-lib=ntoskrnl");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();

    let buf = bindgen::Builder::default()
        .header("wrapper/wrapper.h")
        .use_core()
        .override_abi(Abi::Stdcall, r"\w*")
        .override_abi(Abi::C, "__va_start")
        .override_abi(Abi::C, "DbgPrint")
        .override_abi(Abi::C, "DbgPrintEx")
        .override_abi(Abi::C, "DbgPrintReturnControlC")
        .override_abi(Abi::C, "RtlInitializeSidEx")
        .override_abi(Abi::C, r"^[a-z0-9_]+$")
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
        .to_string();

    let buf = r#"use crate::bind::base::*;

"#
    .to_string()
        + buf.as_str();
    std::fs::write("src/bind/ntoskrnl.rs", buf).expect("Fail to write converted bindings!");

    cc::Build::new()
        .flag("/kernel")
        .include(include_dir)
        .file("wrapper/wrapper.c")
        .compile("wrapper_ntoskrnl");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    generate_base();
    generate_ntoskrnl();
}
