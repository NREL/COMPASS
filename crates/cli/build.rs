fn main() {
    //Link against the Windows Restart Manager library
    println!("cargo:rustc-link-lib=dylib=rstrtmgr");
}