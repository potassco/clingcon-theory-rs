use std::env;
fn main() {
    // checkout clingcon
    // git clone https://github.com/potassco/clingcon
    // cd clingcon
    // git checkout v5.0.0
    // copy clingo.h

    // // Configure and generate bindings.
    // let bindings = bindgen::Builder::default()
    //     .header("clingcon/libclingcon/clingcon.h")
    //     .whitelist_type("clingcon_theory_t")
    //     .whitelist_function("clingcon_create")
    //     .whitelist_function("clingcon_destroy")
    //     .whitelist_function("clingcon_register")
    //     .whitelist_function("clingcon_rewrite_ast")
    //     .whitelist_function("clingcon_prepare")
    //     .whitelist_function("clingcon_register_options")
    //     .whitelist_function("clingcon_validate_options")
    //     .whitelist_function("clingcon_on_model")
    //     .whitelist_function("clingcon_on_statistics")
    //     .whitelist_function("clingcon_lookup_symbol")
    //     .whitelist_function("clingcon_get_symbol")
    //     .whitelist_function("clingcon_assignment_begin")
    //     .whitelist_function("clingcon_assignment_next")
    //     .whitelist_function("clingcon_assignment_has_value")
    //     .whitelist_function("clingcon_assignment_get_value")
    //     .whitelist_function("clingcon_configure")
    //     .size_t_is_usize(true)
    //     .generate()
    //     .unwrap();

    // // Write the generated bindings to an output file.
    // bindings.write_to_file("bindings.rs").unwrap();

    let path = env::var("CLINGCON_LIBRARY_PATH").expect("$CLINGCON_LIBRARY_PATH should be defined");
    println!("cargo:rustc-link-search=native={}", path);

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=import_clingcon");
    } else {
        println!("cargo:rustc-link-lib=dylib=clingcon");
    }
}
