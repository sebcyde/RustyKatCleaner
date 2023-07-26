pub mod helper_functions {
    use std::fs;

    pub async fn get_dir_file_paths<'a>(
        path: &'a str,
        file_paths: &'a mut Vec<fs::DirEntry>,
    ) -> Result<&'a mut Vec<fs::DirEntry>, std::io::Error> {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Check if it's a file and add its path to the vector
            if path.is_file() {
                file_paths.push(entry);
            }
        }

        Ok(file_paths) // Return the vector directly without cloning
    }

    pub async fn filter_dirty_directories<'a>(
        files: &'a mut Vec<fs::DirEntry>,
        paths: &'a mut Vec<String>,
    ) -> &'a mut Vec<String> {
        let filtered_names: Vec<String> = files
            .iter()
            .filter(|file| file.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .map(|file| {
                file.file_name()
                    .into_string()
                    .unwrap_or_else(|_| String::new())
            })
            .filter(|name| name.to_lowercase() != "self-healing")
            .collect();

        paths.extend(filtered_names);
        paths
    }

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
