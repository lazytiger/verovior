use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = cmake::Config::new("verovio/cmake");
    let target = env::var("TARGET").expect("TARGET");
    config
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("NO_EDIT_SUPPORT", "ON")
        .define("NO_PAE_SUPPORT", "ON")
        .define("NO_HUMDRUM_SUPPORT", "ON")
        .define("NO_ABC_SUPPORT", "ON");
    if target.contains("windows") {
        config
            .define("CMAKE_CXX_FLAGS", "/utf-8 /MP")
            .define("CMAKE_C_FLAGS", "/utf-8 /MP")
            .define("CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS", "TRUE");
    } else {
        config.generator("Ninja");
    }
    if target.contains("android") {
        let ndk_root = env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME");
        config.define(
            "CMAKE_TOOLCHAIN_FILE",
            format!("{}/build/cmake/android.toolchain.cmake", ndk_root),
        );
        config.define("BUILD_AS_ANDROID_LIBRARY", "ON");
        config.define("ANDROID_PLATFORM", "android-29");
        config.define("ANDROID_ABI", "x86_64");
    } else {
        config.define("BUILD_AS_LIBRARY", "ON");
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
