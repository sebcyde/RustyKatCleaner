pub mod get_dir_file_paths {
    use std::fs;

    pub async fn get_dir_file_paths<'a>(
        path: &String,
        file_paths: &'a mut Vec<fs::DirEntry>,
    ) -> Result<&'a mut Vec<fs::DirEntry>, std::io::Error> {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                file_paths.push(entry);
            }
        }

        Ok(file_paths)
    }
}
