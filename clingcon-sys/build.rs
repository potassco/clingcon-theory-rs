use std::env;
fn main() {
    // checkout clingcon
    // git clone https://github.com/potassco/clingcon
    // cd clingcon
    // git checkout v5.2.0
    // copy clingo.h

    // // Configure and generate bindings.
    // let bindings = bindgen::Builder::default()
    //     .header("clingcon/libclingcon/clingcon.h")
    //     .allowlist_type("clingo_symbol_t")
    //     .allowlist_type("clingcon_value_type_t")
    //     .allowlist_type("clingcon_value__bindgen_ty_1")
    //     .allowlist_type("clingcon_value_t")
    //     .allowlist_type("clingcon_theory")
    //     .allowlist_type("clingcon_theory_t")
    //     .allowlist_type("clingcon_ast_callback_t")
    //     .allowlist_function("clingcon_create")
    //     .allowlist_function("clingcon_register")
    //     .allowlist_function("clingcon_rewrite_ast")
    //     .allowlist_function("clingcon_prepare")
    //     .allowlist_function("clingcon_destroy")
    //     .allowlist_function("clingcon_configure")
    //     .allowlist_function("clingcon_register_options")
    //     .allowlist_function("clingcon_validate_options")
    //     .allowlist_function("clingcon_on_model")
    //     .allowlist_function("clingcon_lookup_symbol")
    //     .allowlist_function("clingcon_get_symbol")
    //     .allowlist_function("clingcon_assignment_begin")
    //     .allowlist_function("clingcon_assignment_next")
    //     .allowlist_function("clingcon_assignment_has_value")
    //     .allowlist_function("clingcon_assignment_get_value")
    //     .allowlist_function("clingcon_on_statistics")
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
