pub mod create_dir {
    pub mod create_dir {
        pub async fn create_dir(new_clean_directory: &str) -> String {
            use tokio::fs;

            match fs::metadata(new_clean_directory).await {
                Ok(_) => {
                    println!("Clean directory already exists.");
                    new_clean_directory.to_string()
                }
                Err(_) => {
                    if let Err(err) = fs::create_dir(new_clean_directory).await {
                        eprintln!("Error creating clean container: {}", err);
                        String::new()
                    } else {
                        println!("Created directory.");
                        println!("New directory: {}", new_clean_directory);
                        new_clean_directory.to_string()
                    }
                }
            }
        }
    }
}
