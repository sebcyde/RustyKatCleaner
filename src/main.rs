use functions::{
    group_files::group_files::GroupedFiles, process_group::process_group::process_group,
};

pub mod functions {
    pub mod check_directory;
    pub mod create_directory;
    pub mod create_excel;
    pub mod create_subdirectory;
    pub mod data_transfer;
    pub mod file_data_reader;
    pub mod file_fisher;
    pub mod file_functions;
    pub mod filter_dirty_directories;
    pub mod get_directory_file_paths;
    pub mod get_file_name;
    pub mod get_test_company;
    pub mod group_files;
    pub mod process_group;
    pub mod result_checker;
    pub mod transfer_directories;
}

#[tokio::main]
async fn main() {
    use chrono;
    use functions::check_directory::check_directory::check_directory;
    use functions::create_excel::create_excel;
    use functions::create_subdirectory::create_subdirectory::create_subdirectory;
    use functions::data_transfer::data_transfer::data_transfer;
    use functions::file_data_reader::file_data_reader::ReaderBuilder;
    use functions::file_functions::file_functions::*;
    // use functions::file_fisher::file_fisher::file_fisher;
    use functions::create_directory::create_dir::create_dir;
    use functions::filter_dirty_directories::filter_dirty_directories::filter_dirty_directories;
    use functions::get_directory_file_paths::get_dir_file_paths::get_dir_file_paths;
    use functions::get_file_name::get_file_name::get_file_name;
    use functions::get_test_company::get_test_company::get_test_company;
    use functions::group_files::group_files::group_files;
    use functions::group_files::group_files::GroupedFiles;
    use functions::process_group::process_group::process_group;
    use functions::result_checker::result_checker::result_checker;
    use functions::transfer_directories::transfer_directories::transfer_directories;
    use std::fs;

    struct Paths {
        dirty: String,
        clean: String,
        logs: String,
    }

    struct TestResults {
        total: &'static mut u32,
        passed: &'static mut u32,
        failed: &'static mut u32,
        skipped: &'static mut u32,
    }

    let _home_paths: Paths = Paths {
        dirty: String::from("C://Users/SebCy/Documents/Documents/Work/Katalon_Dirty"),
        clean: String::from("C://Users/SebCy/Documents/Documents/Work/Katalon_Clean"),
        logs: String::from("C://Users/SebCy/Documents/Documents/Work/Katalon_Cleaner_Logs"),
    };

    let work_paths: Paths = Paths {
        dirty: String::from("C:/Users/sebastian.cyde/Documents/Other/Katalon/UK Site Tests/flightclub-testing/Reports"),        
        clean: String::from("C:/Users/sebastian.cyde/Documents/Other/Katalon/UK Site Tests/flightclub-testing/Reports"),
        logs: String::from("C:/Users/sebastian.cyde/Documents/Other/CleanerLogs"),
    };

    println!(" ");

    // TODO - pattern matcher for user input here
    // main_converter(home_paths.dirty, home_paths.clean, home_paths.logs).await;
    // main_converter(work_paths.dirty, work_paths.clean, work_paths.logs).await;

    let dir_dirty: String = String::from(
        "C:/Users/sebastian.cyde/Documents/Other/Katalon/UK Site Tests/flightclub-testing/Reports",
    );
    let dir_clean: String = String::from(
        "C:/Users/sebastian.cyde/Documents/Other/Katalon/UK Site Tests/flightclub-testing/Reports",
    );
    let dir_logs: String = String::from("C:/Users/sebastian.cyde/Documents/Other/CleanerLogs");

    let formatted_date: String = format!("{}", chrono::Utc::now().format("%d%m%Y_%H%M%S"));
    println!("Current date: {}", chrono::Utc::now());
    println!("Formatted date: {}", formatted_date);
    println!(" ");

    // Filtering files to be transformed
    let mut dirty_file_paths: &mut Vec<fs::DirEntry> = &mut Vec::new();
    let mut file_paths: &mut Vec<String> = &mut Vec::new();

    let path_result = get_dir_file_paths(&dir_dirty, &mut dirty_file_paths).await;
    result_checker(path_result).await;

    let mut files: &mut Vec<fs::DirEntry> = &mut dirty_file_paths;
    let filtered_directories = filter_dirty_directories(&mut files, &mut file_paths).await;
    println!("Filtered Directories: {:?}", filtered_directories.len());
    println!("They are: {:?}", filtered_directories);
    println!(" ");

    // Creating end result container
    println!("Creating end directory");
    let new_clean_dir: String =
        dir_clean.clone().to_owned() + "/" + &formatted_date.clone().to_owned();
    println!("New clean directory: {}", new_clean_dir);

    check_directory(&new_clean_dir).await;
    create_subdirectory(&new_clean_dir).await;

    // TODO - Initiate logs

    // Grouping related files
    let all_files: Vec<String> = transfer_directories(filtered_directories, &dir_dirty).await;
    println!("Directories to clean: {}", all_files.len());

    let grouped_files: Vec<GroupedFiles> = group_files(all_files).await;
    println!("Grouped Files: {}", grouped_files.len());

    let _failed_tests: Vec<String> = Vec::new();
    println!(" ");

    // let Results: TestResults = TestResults {
    //     total: &mut (grouped_files.len() as u32),
    //     passed: &mut 0,
    //     failed: &mut 0,
    //     skipped: &mut 0,
    // };

    GroupedFiles.into_iter().for_each(|group, index| {
        process_group(new_clean_directory, group, formatted_date, results, index).await;
    });

    // for (const [index, Group] of GroupedFiles.entries()) {
    //     await processGroup(Group, index);
    //   }

    //   await updateLogs(TestResults, LogPath, failedTests);
    //   console.log("Results:", TestResults);
}
