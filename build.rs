fn main() {
    // Default search paths...
    println!("cargo:rustc-link-search=C:/Program Files/Lucid Vision Labs/Arena SDK/lib64/ArenaC");

    // Custom install path.
    if let Some(lucid_arena_sdk_lib_path) = option_env!("LUCID_ARENA_SDK_LIB_PATH") {
        println!("cargo:rustc-link-search={}", lucid_arena_sdk_lib_path);
    }

    // Tell cargo to tell rustc to link the Arena shared library.
    println!("cargo:rustc-link-lib=ArenaC_v140");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
}
