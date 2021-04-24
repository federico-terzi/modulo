/*
 * This file is part of modulo.
 *
 * Copyright (C) 2020-2021 Federico Terzi
 *
 * modulo is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * modulo is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with modulo.  If not, see <https://www.gnu.org/licenses/>.
 */

use regex::Regex;
use std::path::{Path, PathBuf};

// TODO: add documentation for windows compile
// Go to %WXWIN%/build/msw
// nmake /f makefile.vc BUILD=release TARGET_CPU=X64

#[cfg(target_os = "windows")]
fn build_native() {
    let modulo_sys_location = std::env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR env variable");
    let wx_path = PathBuf::from(&modulo_sys_location).parent().unwrap().join("vendor").join("wxWidgets");
    eprintln!("{:?}", wx_path);
    if !wx_path.is_dir() {
        panic!("The given WXWIN directory is not valid");
    }

    // Make sure wxWidgets is compiled
    if !wx_path
        .join("build")
        .join("msw")
        .join("vc_mswu_x64")
        .is_dir()
    {
        panic!("wxWidgets is not compiled correctly, missing 'build/msw/vc_mswu_x64' directory")
    }

    let wx_include_dir = wx_path.join("include");
    let wx_include_msvc_dir = wx_include_dir.join("msvc");
    let wx_lib_dir = wx_path.join("lib").join("vc_x64_lib");

    cc::Build::new()
        .cpp(true)
        .file("native/form.cpp")
        .file("native/search.cpp")
        .file("native/common.cpp")
        .flag("/EHsc")
        .include(wx_include_dir)
        .include(wx_include_msvc_dir)
        .compile("modulosys");

    // Add resources (manifest)
    let mut resources = winres::WindowsResource::new();
    resources.set_manifest(include_str!("res/win.manifest"));
    resources
        .compile()
        .expect("unable to compile resource file");

    println!(
        "cargo:rustc-link-search=native={}",
        wx_lib_dir.to_string_lossy()
    );
}

// TODO: add documentation for macos
// Install LLVM:
// brew install llvm
// Compile wxWidgets:
// mkdir build-cocoa
// cd build-cocoa
// ../configure --disable-shared --enable-macosx_arch=x86_64
// make -j6
//
// Run
// WXMAC=/Users/freddy/wxWidgets cargo run
#[cfg(target_os = "macos")]
fn build_native() {
    let wx_location = std::env::var("WXMAC").expect("unable to find wxWidgets directory, please add a WXMAC env variable with the absolute path");
    let wx_path = PathBuf::from(&wx_location);
    println!("{}", wx_location);
    if !wx_path.is_dir() {
        panic!("The given WXMAC directory is not valid");
    }

    // Make sure wxWidgets is compiled
    if !wx_path.join("build-cocoa").is_dir() {
        panic!("wxWidgets is not compiled correctly, missing 'build-cocoa/' directory")
    }

    let config_path = wx_path.join("build-cocoa").join("wx-config");
    let cpp_flags = get_cpp_flags(&config_path);

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("native/form.cpp")
        .file("native/common.cpp")
        .file("native/search.cpp")
        .file("native/mac.mm");
    build.flag("-std=c++17");

    for flag in cpp_flags {
        build.flag(&flag);
    }

    build.compile("modulosys");

    // Render linker flags

    generate_linker_flags(&config_path);

    // On (older) OSX we need to link against the clang runtime,
    // which is hidden in some non-default path.
    //
    // More details at https://github.com/alexcrichton/curl-rust/issues/279.
    if let Some(path) = macos_link_search_path() {
        println!("cargo:rustc-link-lib=clang_rt.osx");
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(not(target_os = "windows"))]
fn get_cpp_flags(wx_config_path: &Path) -> Vec<String> {
    let config_output = std::process::Command::new(&wx_config_path)
        .arg("--cxxflags")
        .output()
        .expect("unable to execute wx-config");
    let config_libs =
        String::from_utf8(config_output.stdout).expect("unable to parse wx-config output");
    let cpp_flags: Vec<String> = config_libs
        .split(' ')
        .filter_map(|s| {
            if !s.trim().is_empty() {
                Some(s.trim().to_owned())
            } else {
                None
            }
        })
        .collect();
    cpp_flags
}

#[cfg(not(target_os = "windows"))]
fn generate_linker_flags(wx_config_path: &Path) {
    let config_output = std::process::Command::new(&wx_config_path)
        .arg("--libs")
        .output()
        .expect("unable to execute wx-config libs");
    let config_libs =
        String::from_utf8(config_output.stdout).expect("unable to parse wx-config libs output");
    let linker_flags: Vec<String> = config_libs
        .split(' ')
        .filter_map(|s| {
            if !s.trim().is_empty() {
                Some(s.trim().to_owned())
            } else {
                None
            }
        })
        .collect();

    let static_lib_extract = Regex::new(r"lib/lib(.*)\.a").unwrap();

    // Translate the flags generated by `wx-config` to commands
    // that cargo can understand.
    for (i, flag) in linker_flags.iter().enumerate() {
        if flag.starts_with("-L") {
            let path = flag.trim_start_matches("-L");
            println!("cargo:rustc-link-search=native={}", path);
        } else if flag.starts_with("-framework") {
            println!("cargo:rustc-link-lib=framework={}", linker_flags[i + 1]);
        } else if flag.starts_with('/') {
            let captures = static_lib_extract
                .captures(flag)
                .expect("unable to capture flag regex");
            let libname = captures.get(1).expect("unable to find static libname");
            println!("cargo:rustc-link-lib=static={}", libname.as_str());
        } else if flag.starts_with("-l") {
            let libname = flag.trim_start_matches("-l");
            println!("cargo:rustc-link-lib=dylib={}", libname);
        }
    }
}

// Taken from curl-rust: https://github.com/alexcrichton/curl-rust/pull/283/files
#[cfg(target_os = "macos")]
fn macos_link_search_path() -> Option<String> {
    let output = std::process::Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        println!(
            "failed to run 'clang --print-search-dirs', continuing without a link search path"
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').skip(1).next()?;
            return Some(format!("{}/lib/darwin", path));
        }
    }

    println!("failed to determine link search path, continuing without it");
    None
}

// TODO: add documentation for linux
// Install LLVM:
// sudo apt install clang
// Install wxWidgets:
// sudo apt install libwxgtk3.0-0v5 libwxgtk3.0-dev
//
// cargo run
#[cfg(target_os = "linux")]
fn build_native() {
    // Make sure wxWidgets is installed
    if std::process::Command::new("wx-config")
        .arg("--version")
        .output()
        .is_err()
    {
        panic!("wxWidgets is not installed, as `wx-config` cannot be exectued")
    }

    let config_path = PathBuf::from("wx-config");
    let cpp_flags = get_cpp_flags(&config_path);

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("native/form.cpp")
        .file("native/search.cpp")
        .file("native/common.cpp");
    build.flag("-std=c++17");

    for flag in cpp_flags {
        build.flag(&flag);
    }

    build.compile("modulosys");

    // Render linker flags

    generate_linker_flags(&config_path);
}

fn main() {
    // TODO: might need to add rerun if changed: https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorerun-if-changedpath

    build_native();
}
