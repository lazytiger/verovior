use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = cmake::Config::new("verovio/cmake");
    config
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_AS_LIBRARY", "ON");
    #[cfg(target_os = "windows")]
    {
        config
            .define("CMAKE_CXX_FLAGS", "/utf-8 /MP")
            .define("CMAKE_C_FLAGS", "/utf-8 /MP")
            .define("CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS", "TRUE");
    }
    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=verovio");

    let bindings = bindgen::Builder::default()
        .header("verovio/tools/c_wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-Iverovio/src")
        .clang_arg("-Iverovio/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
