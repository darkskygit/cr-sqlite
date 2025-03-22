fn main() {
    cc::Build::new()
        .files(&[
            "c/crsqlite.c",
            "c/changes-vtab.c",
            "c/ext-data.c",
            "c/sqlite/sqlite3.c",
            "c/core_init.c",
        ])
        .flag("-std=c99")
        .flag("-fPIC")
        .flag("-Wall")
        .flag("-g")
        .define("HAVE_GETHOSTUUID", "0")
        .include("c")
        .include("c/sqlite")
        .compile("crsqlite");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=crsqlite");
}
