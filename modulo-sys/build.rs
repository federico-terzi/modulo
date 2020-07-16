use std::path::PathBuf;

// TODO: add documentation for windows compile
// Go to %WXWIN%/build/msw
// nmake /f makefile.vc BUILD=release TARGET_CPU=X86

#[cfg(target_os = "windows")]
fn build_native() {
    let wx_location = std::env::var("WXWIN").expect("unable to find wxWidgets directory, please add a WXWIN env variable with the absolute path");
    let wx_path = PathBuf::from(&wx_location);
    println!("{}", wx_location);
    if !wx_path.is_dir() {
        panic!("The given WXWIN directory is not valid");
    }

    // Make sure wxWindows is compiled
    if !wx_path.join("build").join("msw").join("vc_mswu").is_dir() {
        panic!("wxWidgets is not compiled correctly, missing 'build/msw/vc_mswu' directory")
    }

    let wx_include_dir = wx_path.join("include");
    let wx_include_msvc_dir = wx_include_dir.join("msvc");
    let wx_lib_dir = wx_path.join("lib").join("vc_lib");

    cc::Build::new()
        .cpp(true)
        .file("native/form.cpp")
        .flag("/EHsc")
        .include(wx_include_dir)
        .include(wx_include_msvc_dir)
        .compile("modulosys");

    // Add resources (manifest)
    let mut resources = winres::WindowsResource::new();
    resources.set_manifest(include_str!("res/win.manifest"));
   
    println!("cargo:rustc-link-search=native={}", wx_lib_dir.to_string_lossy());
}

fn main() {
    build_native()
}