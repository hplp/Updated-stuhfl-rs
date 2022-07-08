// build.rs

extern crate bindgen;
extern crate cc;

fn main() {
    // Get C souce files
    let src = [
        "vendor/stuhfl/src/platform/stuhfl_bl_posix.c",
        "vendor/stuhfl/src/platform/stuhfl_bl_win32.c",
        "vendor/stuhfl/src/platform/stuhfl_platform.c",
        "vendor/stuhfl/src/stuhfl_al.c",
        "vendor/stuhfl/src/stuhfl_dl.c",
        "vendor/stuhfl/src/stuhfl_evalAPI_host.c",
        "vendor/stuhfl/src/stuhfl_helpers.c",
        "vendor/stuhfl/src/stuhfl_log.c",
        "vendor/stuhfl/src/stuhfl_pl.c",
        "vendor/stuhfl/src/stuhfl_sl.c",
    ];

    // Get C include directories
    let include = ["vendor/stuhfl/include", "vendor/stuhfl/include/platform"];

    // Detect OS type for compilation
    let def_os = if cfg!(windows) {
        if cfg!(target_pointer_width = "32") {
            Some("WIN32")
        } else if cfg!(target_pointer_width = "64") {
            Some("WIN64")
        } else {
            None
        }
    } else if cfg!(unix) {
        Some("POSIX")
    } else {
        None
    }
    .expect("Couldn't detect OS type!");

    // Compile library
    cc::Build::new()
        .files(src.iter())
        .includes(include.iter())
        .define(def_os, None)
        .flag("-w")
        .compile("stuhfl");

    // Recompile program if library is modified
    println!("cargo:rerun-if-changed=vendor/stuhfl");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Create Rust bindings of STUHFL library
    let bindings = bindgen::Builder::default()
        // all public functions should be included in the wrapper header
        .header("wrapper.h")
        // clang arguments (mostly include's or define's)
        .clang_args(
            [
                "-Ivendor/stuhfl/include",
                "-Ivendor/stuhfl/include/platform",
                &("-D".to_owned() + def_os),
            ]
            .iter(),
        )
        // functions to be exported into the bindings
        // their associated types will be implicitely included
        .allowlist_function("Connect")
        .allowlist_function("Disconnect")
        .allowlist_function("Get_.*")
        .allowlist_function("Set_.*")
        .allowlist_function("Reboot")
        .allowlist_function("EnterBootloader")
        .allowlist_function("TuneChannel")
        .allowlist_function("Gen2_.*")
        .allowlist_function("Gb29768_.*")
        .allowlist_function("Iso6b_.*")
        .allowlist_function("Inventory_.*")
        // variables to be exported into the bindings
        .allowlist_var("STUHFL_.*")
        // automatically tell cargo to update if the files are changed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // generate the bindings
        .generate()
        // error occured
        .expect("Unable to generate bindings");

    // grab the output from the $OUT_DIR envar
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // write the bindings to the $OUT_DIR
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
