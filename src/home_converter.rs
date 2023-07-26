pub mod home_converter {
    use crate::helper_functions::helper_functions::*;
    use std::fs;

    pub async fn home_converter() {
        let dir_dirty: &str = "C://Users/SebCy/Documents/Documents/Work/Katalon_Dirty";
        let _dir_clean: &str = "C://Users/SebCy/Documents/Documents/Work/Katalon_Clean";
        let _dir_logs: &str = "C://Users/SebCy/Documents/Documents/Work/Katalon_Cleaner_Logs";

        // Filtering files to be transformed
        let mut dirty_file_paths: &mut Vec<fs::DirEntry> = &mut Vec::new();
        let mut file_paths: &mut Vec<String> = &mut Vec::new();

        let path_result = get_dir_file_paths(dir_dirty, &mut dirty_file_paths).await;
        result_checker(path_result).await;

        let mut files: &mut Vec<fs::DirEntry> = &mut dirty_file_paths;
        let filtered_directories = filter_dirty_directories(&mut files, &mut file_paths).await;
        println!("Filtered Directories: {:?}", filtered_directories);
    }
}
