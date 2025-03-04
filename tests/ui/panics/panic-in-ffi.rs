// run-fail
// exec-env:RUST_BACKTRACE=0
// check-run-results
// error-pattern: panic in a function that cannot unwind
// normalize-stderr-test: "\n +[0-9]+:[^\n]+" -> ""
// normalize-stderr-test: "\n +at [^\n]+" -> ""
// needs-unwind
// ignore-emscripten "RuntimeError" junk in output
#![feature(c_unwind)]

extern "C" fn panic_in_ffi() {
    panic!("Test");
}

fn main() {
    panic_in_ffi();
}
