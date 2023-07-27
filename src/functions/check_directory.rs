pub mod check_directory {
    pub async fn check_directory(new_clean_directory: &str) {
        use tokio::fs;

        match fs::metadata(new_clean_directory).await {
            Ok(_) => {
                println!("Clean directory already exists.");
            }
            Err(_) => {
                if let Err(err) = fs::create_dir(new_clean_directory).await {
                    eprintln!("Error creating clean container: {}", err);
                } else {
                    println!("Created directory.");
                    println!("New directory: {}", new_clean_directory)
                }
            }
        }
    }
}
