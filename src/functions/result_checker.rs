pub mod result_checker {
    use std::error::Error;

    pub async fn result_checker<T, E>(result: Result<T, E>)
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        if let Err(err) = result {
            eprintln!("Error: {}", err.into());
            return;
        }
    }
}
