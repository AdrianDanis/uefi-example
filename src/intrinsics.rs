// This is not (at the time of writing) yet implemented in the compiler_builtins crate and
// is for some reason wanted occasionally when linking. It doesn't seem to end up getting
// called so we just shove this here to make builds work
#[no_mangle]
pub extern "C" fn __floatundisf() {
    unimplemented!();
}

