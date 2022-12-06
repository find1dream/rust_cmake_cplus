use cmake;
use bindgen;

fn main(){
    // designate path of CMakeLists.txt
    let dist = cmake::Config::new("src/cpp").build();
    println!("cargo:rustc-link-search=native={}", dist.display());

    // if no other static likbary
    println!("cargo:rustc-link-lib=static=cmake_project");

    // add this if it's c++ source code
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // configure bindgen
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/cpp/wrapper.hpp");

    let bindings = bindgen::Builder::default()
        .header("src/cpp/wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
