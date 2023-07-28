pub mod file_functions {
    use tokio::fs;

    pub async fn file_copier<T: AsRef<str>>(
        from_path: T,
        to_path: T,
        extension: &str,
    ) -> Option<()> {
        println!("Copying {} file", extension);
        println!("Copying from: {}", from_path.as_ref());
        println!("Copying to: {}", to_path.as_ref());
        println!(" ");

        let to_path_with_extension: String = format!("{}.{}", to_path.as_ref(), extension);

        if let Err(error) = fs::copy(from_path.as_ref(), to_path_with_extension).await {
            eprintln!("Error copying {}: {}", extension, error);
            return None;
        }

        Some(())
    }
}
