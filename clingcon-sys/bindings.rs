/* automatically generated by rust-bindgen 0.57.0 */

pub type __uint32_t = ::std::os::raw::c_uint;
pub type __uint64_t = ::std::os::raw::c_ulong;
#[doc = "! Represents a symbol."]
#[doc = "!"]
#[doc = "! This includes numbers, strings, functions (including constants when"]
#[doc = "! arguments are empty and tuples when the name is empty), <tt>\\#inf</tt> and <tt>\\#sup</tt>."]
pub type clingo_symbol_t = u64;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_statistic {
//     _unused: [u8; 0],
// }
// #[doc = "! Handle for the solver statistics."]
// pub type clingo_statistics_t = clingo_statistic;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_model {
//     _unused: [u8; 0],
// }
// #[doc = "! Object representing a model."]
// pub type clingo_model_t = clingo_model;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_ast {
//     _unused: [u8; 0],
// }
// #[doc = "! This struct provides a view to nodes in the AST."]
// pub type clingo_ast_t = clingo_ast;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_control {
//     _unused: [u8; 0],
// }
// #[doc = "! Control object holding grounding and solving state."]
// pub type clingo_control_t = clingo_control;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_options {
//     _unused: [u8; 0],
// }
// #[doc = "! Object to add command-line options."]
// pub type clingo_options_t = clingo_options;
#[doc = "! Corresponding type to ::clingcon_value_type."]
pub type clingcon_value_type_t = ::std::os::raw::c_int;
#[doc = "! Struct to store values that can be returned by a theory."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clingcon_value {
    pub type_: clingcon_value_type_t,
    pub __bindgen_anon_1: clingcon_value__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union clingcon_value__bindgen_ty_1 {
    pub int_number: ::std::os::raw::c_int,
    pub double_number: f64,
    pub symbol: clingo_symbol_t,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_clingcon_value__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<clingcon_value__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(clingcon_value__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<clingcon_value__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(clingcon_value__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<clingcon_value__bindgen_ty_1>())).int_number as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingcon_value__bindgen_ty_1),
            "::",
            stringify!(int_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<clingcon_value__bindgen_ty_1>())).double_number as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingcon_value__bindgen_ty_1),
            "::",
            stringify!(double_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<clingcon_value__bindgen_ty_1>())).symbol as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingcon_value__bindgen_ty_1),
            "::",
            stringify!(symbol)
        )
    );
}
#[test]
fn bindgen_test_layout_clingcon_value() {
    assert_eq!(
        ::std::mem::size_of::<clingcon_value>(),
        16usize,
        concat!("Size of: ", stringify!(clingcon_value))
    );
    assert_eq!(
        ::std::mem::align_of::<clingcon_value>(),
        8usize,
        concat!("Alignment of ", stringify!(clingcon_value))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<clingcon_value>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingcon_value),
            "::",
            stringify!(type_)
        )
    );
}
#[doc = "! Struct to store values that can be returned by a theory."]
pub type clingcon_value_t = clingcon_value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clingcon_theory {
    _unused: [u8; 0],
}
#[doc = "! The clingcon theory."]
pub type clingcon_theory_t = clingcon_theory;
#[doc = "! Callback to rewrite statements (see ::clingcon_rewrite_ast)."]
pub type clingcon_ast_callback_t = ::std::option::Option<
    unsafe extern "C" fn(ast: *mut clingo_ast_t, data: *mut ::std::os::raw::c_void) -> bool,
>;
extern "C" {
    #[doc = "! Creates the theory."]
    pub fn clingcon_create(theory: *mut *mut clingcon_theory_t) -> bool;
}
extern "C" {
    #[doc = "! Register the theory with a control object."]
    pub fn clingcon_register(
        theory: *mut clingcon_theory_t,
        control: *mut clingo_control_t,
    ) -> bool;
}
extern "C" {
    #[doc = "! Rewrite asts before adding them via the given callback."]
    pub fn clingcon_rewrite_ast(
        theory: *mut clingcon_theory_t,
        ast: *mut clingo_ast_t,
        add: clingcon_ast_callback_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    #[doc = "! Prepare the theory between grounding and solving."]
    pub fn clingcon_prepare(theory: *mut clingcon_theory_t, control: *mut clingo_control_t)
        -> bool;
}
extern "C" {
    #[doc = "! Destroy the theory."]
    #[doc = "!"]
    #[doc = "! Currently no way to unregister a theory."]
    pub fn clingcon_destroy(theory: *mut clingcon_theory_t) -> bool;
}
extern "C" {
    #[doc = "! Configure theory manually (without using clingo's options facility)."]
    #[doc = "!"]
    #[doc = "! Note that the theory has to be configured before registering it and cannot"]
    #[doc = "! be reconfigured."]
    pub fn clingcon_configure(
        theory: *mut clingcon_theory_t,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = "! Register options of the theory."]
    pub fn clingcon_register_options(
        theory: *mut clingcon_theory_t,
        options: *mut clingo_options_t,
    ) -> bool;
}
extern "C" {
    #[doc = "! Validate options of the theory."]
    pub fn clingcon_validate_options(theory: *mut clingcon_theory_t) -> bool;
}
extern "C" {
    #[doc = "! Callback for models."]
    pub fn clingcon_on_model(theory: *mut clingcon_theory_t, model: *mut clingo_model_t) -> bool;
}
extern "C" {
    #[doc = "! Obtain a symbol index which can be used to get the value of a symbol."]
    #[doc = "!"]
    #[doc = "! Returns true if the symbol exists."]
    #[doc = "! Does not throw."]
    pub fn clingcon_lookup_symbol(
        theory: *mut clingcon_theory_t,
        symbol: clingo_symbol_t,
        index: *mut usize,
    ) -> bool;
}
extern "C" {
    #[doc = "! Obtain the symbol at the given index."]
    #[doc = "!"]
    #[doc = "! Does not throw."]
    pub fn clingcon_get_symbol(theory: *mut clingcon_theory_t, index: usize) -> clingo_symbol_t;
}
extern "C" {
    #[doc = "! Initialize index so that it can be used with clingcon_assignment_next."]
    #[doc = "!"]
    #[doc = "! Does not throw."]
    pub fn clingcon_assignment_begin(
        theory: *mut clingcon_theory_t,
        thread_id: u32,
        index: *mut usize,
    );
}
extern "C" {
    #[doc = "! Move to the next index that has a value."]
    #[doc = "!"]
    #[doc = "! Returns true if the updated index is valid."]
    #[doc = "! Does not throw."]
    pub fn clingcon_assignment_next(
        theory: *mut clingcon_theory_t,
        thread_id: u32,
        index: *mut usize,
    ) -> bool;
}
extern "C" {
    #[doc = "! Check if the symbol at the given index has a value."]
    #[doc = "!"]
    #[doc = "! Does not throw."]
    pub fn clingcon_assignment_has_value(
        theory: *mut clingcon_theory_t,
        thread_id: u32,
        index: usize,
    ) -> bool;
}
extern "C" {
    #[doc = "! Get the symbol and it's value at the given index."]
    #[doc = "!"]
    #[doc = "! Does not throw."]
    pub fn clingcon_assignment_get_value(
        theory: *mut clingcon_theory_t,
        thread_id: u32,
        index: usize,
        value: *mut clingcon_value_t,
    );
}
extern "C" {
    #[doc = "! Callback for statistic updates."]
    #[doc = "!"]
    #[doc = "! Best add statistics under a subkey with the name of your theory."]
    pub fn clingcon_on_statistics(
        theory: *mut clingcon_theory_t,
        step: *mut clingo_statistics_t,
        accu: *mut clingo_statistics_t,
    ) -> bool;
}
