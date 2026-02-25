#![allow(unsafe_code)]

::pgrx::pgrx_embed!();

#[macro_export]
macro_rules! schema_generation {
    ($($symbol:ident)*; $($import:ident)*) => {
        pub fn main() -> Result<(), Box<dyn std::error::Error>> {
            $(
                const _: () = {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn $import() {
                        panic!("{} is called unexpectedly.", stringify!($import));
                    }
                };
            )*

            extern crate pg_tokenizer as _;

            use ::pgrx::pgrx_sql_entity_graph::ControlFile;
            use ::pgrx::pgrx_sql_entity_graph::PgrxSql;
            use ::pgrx::pgrx_sql_entity_graph::SqlGraphEntity;

            let p = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/pg_tokenizer.control"));
            let control_file = ControlFile::try_from(p)?;

            unsafe extern "Rust" {
                $(safe fn $symbol() -> SqlGraphEntity;)*
            }

            let mut e = vec![SqlGraphEntity::ExtensionRoot(control_file)];
            $(e.push($symbol());)*

            let pgrx_sql = PgrxSql::build(e.into_iter(), "pg_tokenizer".to_string(), false)?;
            pgrx_sql.write(&mut std::io::stdout())?;

            Ok(())
        }
    };
}
