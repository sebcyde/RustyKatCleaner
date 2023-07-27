pub mod filter_dirty_directories {
    use std::fs;

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
}
