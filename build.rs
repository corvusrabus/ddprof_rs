fn main() {
    println!("cargo:rustc-link-lib=static=dd_profiling");
    println!("cargo:rustc-link-search=native=ddprof/lib");
}