#![recursion_limit = "1024"]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let code = whatisthis::cli_main();
    std::process::exit(code);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use console_error_panic_hook::set_once as set_panic_hook;
    set_panic_hook();
}
#[cfg(target_arch = "wasm32")]
pub use whatisthis::wasm::process_file;
