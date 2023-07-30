pub mod process_group {
    use crate::functions::{
        check_directory::check_directory::check_directory,
        create_directory::create_dir::create_dir::create_dir, create_excel::create_excel,
        data_transfer::data_transfer::data_transfer, file_functions::file_functions::file_copier,
        get_file_name::get_file_name::get_file_name,
        get_test_company::get_test_company::get_test_company,
        group_files::group_files::GroupedFiles,
    };

    struct TestResults {
        total: &'static mut u32,
        passed: &'static mut u32,
        failed: &'static mut u32,
        skipped: &'static mut u32,
    }

    pub async fn process_group(
        new_clean_directory: String,
        group: GroupedFiles,
        formatted_date: String,
        results: TestResults,
    ) {
        // Get group's company for folder organisation
        let file_name: String = get_file_name().await;
        let company: Option<String> = get_test_company().await;

        println!("File Name: {}", file_name);
        println!("Company: {}", company.clone().unwrap());

        // Skip if no company found
        // match company {
        //     None => {
        //         *results.skipped += 1;
        //         return;
        //     }
        //     _ => (),
        // }

        // create containing directory
        // let section_path: String = check_directory(
        //     &(new_clean_directory.to_owned() + "/" + company.to_owned() + "/" + group.section),
        // )
        // .await;

        let section_path: String = create_dir(
            &(new_clean_directory + "/" + &company.unwrap() + "/" + &group.section.unwrap()),
        )
        .await;

        let date_path: String =
            create_dir(&(section_path.to_owned() + "/" + &file_name.to_owned())).await;

        let day_path: String =
            create_dir(&(date_path.to_owned() + "/" + &file_name.split('_').next().unwrap())).await;

        let directory: String = create_dir(&(day_path.to_owned() + "/" + &formatted_date)).await;

        println!("Created containing directory.");
        println!(" ");

        // Copy over misc files
        let pdf: String = group.pdf.unwrap();
        let html: String = group.html.unwrap();
        let csv: String = group.csv.unwrap();
        let section = group.section.unwrap();

        if let Some(pdf) = file_copier(pdf, directory + "/" + &file_name, "pdf").await {
            println!("PDF copied successfully.");
        } else {
            println!("Failed to copy the PDF.");
        }

        if let Some(html) = file_copier(html, directory + "/" + &file_name, "html").await {
            println!("HTML copied successfully.");
        } else {
            println!("Failed to copy the HTML.");
        }

        // if let Some(csv) = file_copier(csv, directory + "/" + &file_name, "csv").await {
        //     println!("CSV copied successfully.");
        // } else {
        //     println!("Failed to copy the CSV.");
        // }

        // create excel stuff
        let excel_data: Result<(), Box<dyn Error>> = create_excel(&directory, &file_name).await;

        // Transfer data to new excel sheet
        let result: bool = data_transfer(
            &csv,
            &section,
            excel_data.workbook,
            excel_data.worksheet,
            excel_data.path,
        )
        .await
        .unwrap();

        if !result {
            println!("Data transfer failed.");
            results.failed += 1;
        } else {
            println!("Data transfer successful");
            results.passed += 1;
        }

        //       await addToLog(LogPath, Group, Result);

        if result {
            TestResults.Passed += 1;
        } else {
            TestResults.Failed += 1;
        }
    }
}
