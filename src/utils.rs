use pgrx::{pg_sys::panic::ErrorReportable, spi::Query, FromDatum, IntoDatum};

pub fn spi_get_one<T>(query: &str, args: <&str as Query<'static>>::Arguments) -> Option<T>
where
    T: FromDatum + IntoDatum,
{
    pgrx::Spi::connect(|client| {
        let tuptable = client.select(query, Some(1), args).unwrap_or_report();

        match tuptable.first().get_one::<T>() {
            Ok(Some(bytes)) => Some(bytes),
            Ok(None) => panic!("Get null value when excuting spi query: {}", query),
            Err(e) if matches!(e, pgrx::spi::SpiError::InvalidPosition) => {
                return None;
            }
            Err(e) => panic!(
                "Failed to excuting spi query, error: {}, query: {}",
                e, query
            ),
        }
    })
}
