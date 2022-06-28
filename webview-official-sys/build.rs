fn main() {
    // println!("cargo:rustc-link-lib=webview");
    for &lib in &[
        "WebView2Loader.dll",
        "windowsapp",
        "user32",
        "oleaut32",
        "ole32",
    ] {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
// use cc::Build;
// use std::env;
// use std::fs;
// use std::path::{Path, PathBuf};

// fn main() {
//     let mut build = Build::new();

//     let target = env::var("TARGET").unwrap();

//     build
//         .cpp(true)
//         .include("webview-official/webview.h")
//         .flag_if_supported("/std:c++11")
//         .flag_if_supported("-w");

//     // if env::var("DEBUG").is_err() {
//     //     build.define("NDEBUG", None);
//     // } else {
//     //     build.define("DEBUG", None);
//     // }

//     if target.contains("windows") {
//         println!("WINDOWS!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
//         let current_dir = env::current_dir().unwrap();
//         println!("CURRENT_DIR: {}", current_dir.as_path().to_str().unwrap());
//         // build.define("UNICODE", None); // doesn't work atm.
//         build
//             .file("webview-official/webview.cc")
//             .flag_if_supported("/std:c++17");
//         // build.include("webview-official/script");

//         build.include(current_dir.as_path());
//         build.include("/nix/store/jp9bk07xzrajfr7aa47mp92dbc3vpnaa-mingw-w64-x86_64-w64-mingw32-9.0.0-headers/include");
//         build.include(format!(
//             "{}/libs/webview2/build/native/include",
//             current_dir.as_path().to_str().unwrap()
//         ));
//         build.include(format!(
//             "{}/libs/webview2/build/native/x64",
//             current_dir.as_path().to_str().unwrap()
//         ));
//         build.define("WEBVIEW_EDGE", None);
//         build.define("WIN32", None);
//         build.define("_WIN32", None);
//         // build.include("WebView2Loaderw.dll");

//         // for &lib in &["windowsapp", "user32", "oleaut32", "ole32"] {
//         //     println!("cargo:rustc-link-lib={}", lib);
//         // }

//         // let webview2_arch = if target.contains("x86_64") {
//         //     "x64"
//         // } else {
//         //     "x86"
//         // };

//         let current_dir = env::current_dir().unwrap();
//         // let mut path_to_dll = current_dir.clone();
//         // path_to_dll.push("WebView2Loader.dll");
//         println!(
//             "cargo:rustc-link-search={}",
//             current_dir.as_path().to_str().unwrap()
//         );
//         println!(
//             "cargo:rustc-link-search={}",
//             "/nix/store/jp9bk07xzrajfr7aa47mp92dbc3vpnaa-mingw-w64-x86_64-w64-mingw32-9.0.0-headers/include"
//         );
//         println!(
//             "cargo:rustc-link-search={}/libs/webview2/build/native/include",
//             current_dir.as_path().to_str().unwrap()
//         );
//         println!(
//             "cargo:rustc-link-search={}/libs/webview2/build/native/x64",
//             current_dir.as_path().to_str().unwrap()
//         );
//         // println!(
//         //     "cargo:rustc-link-lib={}/WebView2Loader.dll",
//         //     current_dir.as_path().to_str().unwrap()
//         // );
//         // println!(
//         //     "cargo:rustc-link-lib={}/WebView2.h",
//         //     current_dir.as_path().to_str().unwrap()
//         // );
//         println!(
//             "cargo:rustc-link-lib={}/libs/webview2/build/native/x64/WebView2Loader.dll",
//             current_dir.as_path().to_str().unwrap()
//         );
//         println!(
//             "cargo:rustc-link-lib={}/libs/webview2/build/native/include/WebView2.h",
//             current_dir.as_path().to_str().unwrap()
//         );

//         // calculate full path to WebView2Loader.dll
//         // let mut webview2_path_buf = PathBuf::from(env::current_dir().unwrap().to_str().unwrap());
//         // webview2_path_buf.push("webview-official/script");
//         // webview2_path_buf.push(webview2_arch);
//         // let webview2_dir = webview2_path_buf.as_path().to_str().unwrap();

//         // let loader_asm_name = "WebView2Loader.dll";

//         // println!("cargo:rustc-link-search={}", webview2_dir);
//         // println!("cargo:rustc-link-lib={}", loader_asm_name);

//         // // copy WebView2Loader.dll to `target/debug`
//         // let mut src_asm_buf = PathBuf::from(webview2_dir);
//         // src_asm_buf.push(loader_asm_name);

//         // // we want to be able to calculate C:\crate\root\target\debug\
//         // //           while we can get this ^^^^^^^^^^^^^   and  ^^^^^ from env::current_dir() and %PROFILE% respectively
//         // // there's no way to get this (reliably)         ^^^^^^
//         // // we can, however, use %OUT_DIR% (C:\crate\root\target\debug\build\webview_rust-xxxx\out\)
//         // // and navigate backwards to here  ^^^^^^^^^^^^^^^^^^^^^^^^^^
//         // let mut target_asm_buf = PathBuf::from(env::var("OUT_DIR").unwrap());
//         // target_asm_buf.pop();
//         // target_asm_buf.pop();
//         // target_asm_buf.pop();
//         // target_asm_buf.push(loader_asm_name);

//         // fs::copy(src_asm_buf.as_path(), target_asm_buf.as_path()).unwrap();
//     } else if target.contains("apple") {
//         build.file("webview-official/webview.cc").flag("-std=c++11");

//         println!("cargo:rustc-link-lib=framework=Cocoa");
//         println!("cargo:rustc-link-lib=framework=WebKit");
//     } else if target.contains("linux") || target.contains("bsd") {
//         let lib = pkg_config::Config::new()
//             .atleast_version("2.8")
//             .probe("webkit2gtk-4.0")
//             .unwrap();

//         for path in lib.include_paths {
//             build.include(path);
//         }
//         // pkg_config::Config::new()
//         //     .atleast_version("3.0")
//         //     .probe("gtk+-3.0")
//         //     .unwrap();

//         build.file("webview-official/webview.cc");
//     } else {
//         panic!("Unsupported platform");
//     }

//     println!("cargo:rerun-if-changed=webview-official/webview.h");
//     println!("cargo:rerun-if-changed=webview-official/webview.cc");

//     build.compile("webview");
// }
