extern crate clingcon_sys;
extern crate clingo;
extern crate clingo_sys;
use clingo::{
    ast,
    theory::{Theory, TheoryValue},
    ControlCtx, GenericControl, Id, Model, Options, Statistics, Symbol,
};
use clingo_sys::clingo_ast;
use clingo_sys::clingo_control;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct ConTheory {
    theory: NonNull<clingcon_sys::clingcon_theory>,
}
impl<'a> ConTheory {
    /// creates the theory
    pub fn create() -> ConTheory {
        let mut theory_ptr = std::ptr::null_mut();
        unsafe { clingcon_sys::clingcon_create(&mut theory_ptr) };
        match NonNull::new(theory_ptr) {
            Some(theory) => ConTheory { theory },
            None => panic!("Tried creating NonNull from a null pointer."),
        }
    }
}
impl Drop for ConTheory {
    fn drop(&mut self) {
        let success = unsafe { clingcon_sys::clingcon_destroy(self.theory.as_ptr()) };
        if !success {
            panic!("call clingcon_destroy returned false")
        }
    }
}
/// An iterator over dl theory values.
pub struct ConTheoryAssignment<'a> {
    dl_theory: &'a ConTheory,
    thread_id: Id,
    index: usize,
}
impl<'a> Iterator for ConTheoryAssignment<'a> {
    type Item = (Symbol, TheoryValue);

    fn next(&mut self) -> Option<(Symbol, TheoryValue)> {
        if !unsafe {
            clingcon_sys::clingcon_assignment_next(
                self.dl_theory.theory.as_ptr(),
                self.thread_id.into(),
                &mut self.index,
            )
        } {
            None
        } else if unsafe {
            clingcon_sys::clingcon_assignment_has_value(
                self.dl_theory.theory.as_ptr(),
                self.thread_id.into(),
                self.index,
            )
        } {
            let sym: clingo_sys::clingo_symbol_t = unsafe {
                clingcon_sys::clingcon_get_symbol(self.dl_theory.theory.as_ptr(), self.index)
            };
            let sym = sym.into();
            let value_internal = clingcon_sys::clingcon_value__bindgen_ty_1 { int_number: 0 };
            let mut value = clingcon_sys::clingcon_value {
                type_: 0,
                __bindgen_anon_1: value_internal,
            };
            unsafe {
                clingcon_sys::clingcon_assignment_get_value(
                    self.dl_theory.theory.as_ptr(),
                    self.thread_id.into(),
                    self.index,
                    &mut value,
                )
            };
            match value.type_ {
                0 => Some((
                    sym,
                    TheoryValue::IntNumber(unsafe { value.__bindgen_anon_1.int_number } as u64),
                )),
                1 => Some((
                    sym,
                    TheoryValue::DoubleNumber(unsafe { value.__bindgen_anon_1.double_number }),
                )),
                2 => {
                    let value = unsafe { value.__bindgen_anon_1.symbol };
                    Some((sym, TheoryValue::Symbol(value.into())))
                }
                x => panic!("unexpected ConTheoryValue {}", x),
            }
        } else {
            None
        }
    }
}
impl<'a> Theory<'a> for ConTheory {
    fn assignment(&'a self, thread_id: Id) -> Box<dyn Iterator<Item = (Symbol, TheoryValue)> + 'a> {
        let mut index = 0;
        unsafe {
            clingcon_sys::clingcon_assignment_begin(
                self.theory.as_ptr(),
                thread_id.into(),
                &mut index,
            )
        }
        Box::new(ConTheoryAssignment {
            dl_theory: self,
            thread_id,
            index,
        })
    }
    /// registers the theory with the control
    fn register<Ctx>(&mut self, ctl: &mut GenericControl<Ctx>) -> bool
    where
        Ctx: ControlCtx,
    {
        let nn: NonNull<clingo_control> = ctl.into();
        unsafe { clingcon_sys::clingcon_register(self.theory.as_ptr(), nn.as_ptr()) }
    }
    /// Rewrite statements before adding them via the given callback.
    fn rewrite_statement(
        &mut self,
        stmt: &ast::Statement,
        builder: &mut ast::ProgramBuilder,
    ) -> bool {
        let add = unsafe_program_builder_add;
        let nn: NonNull<clingo_ast> = stmt.into();
        let pb: *mut clingo_sys::clingo_program_builder = builder.into();
        unsafe {
            clingcon_sys::clingcon_rewrite_ast(
                self.theory.as_ptr(),
                nn.as_ptr(),
                Some(add),
                pb as *mut ::std::os::raw::c_void,
            )
        }
    }

    /// prepare the theory between grounding and solving
    fn prepare<Ctx>(&mut self, ctl: &mut GenericControl<Ctx>) -> bool
    where
        Ctx: ControlCtx,
    {
        let nn: NonNull<clingo_control> = ctl.into();
        unsafe { clingcon_sys::clingcon_prepare(self.theory.as_ptr(), nn.as_ptr()) }
    }
    /// add options for your theory
    fn register_options(&mut self, options: &mut Options) -> bool {
        unsafe { clingcon_sys::clingcon_register_options(self.theory.as_ptr(), options.into()) }
    }
    /// validate options for your theory
    fn validate_options(&mut self) -> bool {
        unsafe { clingcon_sys::clingcon_validate_options(self.theory.as_ptr()) }
    }
    /// callback on every model
    fn on_model(&mut self, model: &mut Model) -> bool {
        unsafe { clingcon_sys::clingcon_on_model(self.theory.as_ptr(), model.into()) }
    }
    /// callback on statistic updates
    /// please add a subkey with the name of your theory
    fn on_statistics(&mut self, step: &mut Statistics, accu: &mut Statistics) -> bool {
        unsafe {
            clingcon_sys::clingcon_on_statistics(self.theory.as_ptr(), step.into(), accu.into())
        }
    }
    /// obtain a symbol index which can be used to get the value of a symbol
    /// returns true if the symbol exists
    /// does not throw
    fn lookup_symbol(&mut self, symbol: Symbol, index: &mut usize) -> bool {
        unsafe { clingcon_sys::clingcon_lookup_symbol(self.theory.as_ptr(), symbol.into(), index) }
    }
    /// obtain the symbol at the given index
    /// does not throw
    fn get_symbol(&mut self, index: usize) -> Symbol {
        let sym: clingo_sys::clingo_symbol_t =
            unsafe { clingcon_sys::clingcon_get_symbol(self.theory.as_ptr(), index) };
        sym.into()
    }
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool {
        unsafe {
            clingcon_sys::clingcon_configure(
                self.theory.as_ptr(),
                key.as_ptr() as *const i8,
                value.as_ptr() as *const i8,
            )
        }
    }
}

unsafe extern "C" fn unsafe_program_builder_add(
    statement: *mut clingo_sys::clingo_ast_t,
    data: *mut ::std::os::raw::c_void,
) -> bool {
    let builder = data as *mut clingo_sys::clingo_program_builder;
    clingo_sys::clingo_program_builder_add(builder, statement)
}
