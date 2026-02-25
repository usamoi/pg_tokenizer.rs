#![allow(unsafe_code)]
#![deny(ffi_unwind_calls)]

mod character_filter;
mod model;
mod pre_tokenizer;
mod text_analyzer;
mod token_filter;
mod tokenizer;
mod utils;

pgrx::pg_module_magic!(
    name = c"pg_tokenizer",
    version = {
        const RAW: &str = env!("PG_TOKENIZER_VERSION");
        const BUFFER: [u8; RAW.len() + 1] = {
            let mut buffer = [0u8; RAW.len() + 1];
            let mut i = 0_usize;
            while i < RAW.len() {
                buffer[i] = RAW.as_bytes()[i];
                i += 1;
            }
            buffer
        };
        const STR: &::core::ffi::CStr =
            if let Ok(s) = ::core::ffi::CStr::from_bytes_with_nul(&BUFFER) {
                s
            } else {
                panic!("there are null characters in PG_TOKENIZER_VERSION")
            };
        const { STR }
    }
);

#[pgrx::pg_guard]
#[unsafe(export_name = "_PG_init")]
unsafe extern "C-unwind" fn _pg_init() {
    if !unsafe { pgrx::pg_sys::process_shared_preload_libraries_in_progress } {
        pgrx::error!("pg_tokenizer must be loaded via shared_preload_libraries.");
    }

    pre_tokenizer::init();
    model::init();
}

#[cfg(not(all(target_endian = "little", target_pointer_width = "64")))]
compile_error!("Target is not supported.");
