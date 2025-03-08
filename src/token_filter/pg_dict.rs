use std::ffi::{CStr, CString};

use pgrx::{
    pg_sys::{DatumGetPointer, Int32GetDatum, PointerGetDatum},
    IntoDatum,
};

use super::TokenFilter;

pub struct PgDictTokenFilter {
    dict_oid: pgrx::pg_sys::Oid,
}

impl PgDictTokenFilter {
    pub fn new(name: &str) -> Self {
        let dict_oid = unsafe {
            pgrx::direct_function_call::<pgrx::pg_sys::Oid>(
                pgrx::pg_sys::regdictionaryin,
                &[CString::new(name).unwrap().as_c_str().into_datum()],
            )
        }
        .expect("Cannot cast name to oid in PgDictTokenFilter::new");

        unsafe { pgrx::pg_sys::lookup_ts_dictionary_cache(dict_oid) };

        PgDictTokenFilter { dict_oid }
    }
}

impl TokenFilter for PgDictTokenFilter {
    fn apply(&self, token: String) -> Vec<String> {
        unsafe {
            let dict = pgrx::pg_sys::lookup_ts_dictionary_cache(self.dict_oid)
                .as_mut()
                .unwrap();

            let res = pgrx::pg_sys::FunctionCall3Coll(
                &raw mut dict.lexize,
                pgrx::pg_sys::InvalidOid,
                PointerGetDatum(dict.dictData),
                PointerGetDatum(token.as_ptr().cast()),
                Int32GetDatum(token.len().try_into().unwrap()),
            );
            if res.is_null() {
                // not recognized
                return vec![token];
            }
            let res = DatumGetPointer(res);

            let mut lexeme_ptr: *const pgrx::pg_sys::TSLexeme = res.cast_const().cast();
            let mut results = Vec::new();
            while !(*lexeme_ptr).lexeme.is_null() {
                let str = CStr::from_ptr((*lexeme_ptr).lexeme);
                results.push(str.to_str().unwrap().to_string());
                pgrx::pg_sys::pfree((*lexeme_ptr).lexeme.cast());
                lexeme_ptr = lexeme_ptr.add(1);
            }
            pgrx::pg_sys::pfree(res.cast());

            results
        }
    }
}
