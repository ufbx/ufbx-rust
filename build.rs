use cc;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=ufbx/ufbx.c");
    println!("cargo:rerun-if-changed=ufbx/prelude.c");
    println!("cargo:rerun-if-changed=ufbx/ufbx.h");
    println!("cargo:rerun-if-changed=ufbx/prelude.h");
    cc::Build::new()
        .define("UFBX_CONFIG_HEADER", "\"prelude.h\"")
        .file("ufbx/prelude.c")
        .file("ufbx/ufbx.c")
        .compile("ufbx");
}
