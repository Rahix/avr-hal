fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    maybe_enable_asm();
}

#[rustversion::before(1.59.0)]
fn maybe_enable_asm() {
    //
}

#[rustversion::since(1.59.0)]
fn maybe_enable_asm() {
    // https://github.com/rust-lang/rust/pull/92816
    println!("cargo:rustc-cfg=avr_hal_asm_macro");
}
