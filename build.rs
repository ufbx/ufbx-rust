use cc;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=ufbx/ufbx.c");
    println!("cargo:rerun-if-changed=ufbx/ufbx.h");
    cc::Build::new()
        .file("ufbx/ufbx.c")
        .compile("ufbx");
}
