mod helper_functions;
use std::{error, fs};

#[tokio::main]
async fn main() {
    use crate::helper_functions::helper_functions::*;

    let dir_dirty: &str =
        "C:/Users/sebastian.cyde/Documents/Other/Katalon/UK Site Tests/flightclub-testing/Reports";
    let _dir_clean: &str =
        "C:/Users/sebastian.cyde/Documents/Other/Katalon/UK Site Tests/flightclub-testing/Reports";
    let _dir_logs: &str = "C:/Users/sebastian.cyde/Documents/Other/CleanerLogs";

    // Filtering files to be transformed
    let mut dirty_file_paths: &mut Vec<fs::DirEntry> = &mut Vec::new();
    let mut file_paths: &mut Vec<String> = &mut Vec::new();

    let path_result = get_dir_file_paths(dir_dirty, &mut dirty_file_paths).await;
    result_checker(path_result).await;

    let mut files: &mut Vec<fs::DirEntry> = &mut dirty_file_paths;
    let filtered_directories = filter_dirty_directories(&mut files, &mut file_paths).await;
    println!("Filtered Directories: {:?}", filtered_directories);
}
