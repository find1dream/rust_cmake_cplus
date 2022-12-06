use cmake;
use bindgen;

fn main(){
    let cpp_path = String::from("src/cpp");
    // designate path of CMakeLists.txt
    let dist = cmake::Config::new(&cpp_path).build();
    println!("cargo:rustc-link-search=native={}", dist.display());

    // if no other static likbary
    println!("cargo:rustc-link-lib=static=cmake_project");

    // add this if it's c++ source code
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // configure bindgen
    let wrapper_file = format!("{}/{}", cpp_path, String::from("wrapper.hpp"));
    let binding_file = String::from("src/bindings.rs");
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", wrapper_file);

    let bindings = bindgen::Builder::default()
        .header(&wrapper_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(&binding_file)
        .expect("Couldn't write bindings!");
}
