pub mod process_group {
    use crate::functions::{
        check_directory::check_directory::check_directory,
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
        results: TestResults,
        index: u32,
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
    }
}

// const processGroup = async (Group, index) => {
//     try {
//       // Get Group's company for folder organization
//       const FileName = await getFileName(Group.PDF);
//       const Company = await getTestCompany(Group);
//       if (!Company) {
//         TestResults.Skipped++;
//         return;
//       }

//       // Create containing directory
//       const sectionPath = await createDirectory(`${NewCleanDirectory}/${Company}/${Group.Section}`);
//       const datePath = await createDirectory(`${sectionPath}/${FileName}`);
//       const dayPath = await createDirectory(`${datePath}/${FileName.split("_")[0]}`);
//       const Directory = await createDirectory(`${dayPath}/${formattedDate}`);
//       console.log("Created containing directory");
//       console.log(" ");

//       // Copy over misc files
//       await PDFCopier(Group.PDF, Directory);
//       await HTMLCopier(Group.HTML, Directory);
//       // await CSVCopier(Group.CSV, Directory);

//       // Create excel stuff
//       const ExcelData = await createExcel(Directory, FileName);

//       // Transfer Data to new excel files
//       const Result = await DataTransfer(
//         Group.CSV,
//         Group.Section,
//         ExcelData.Workbook,
//         ExcelData.WorkSheet,
//         ExcelData.ExcelPath
//       );

//       if (!Result) failedTests.push(Directory);

//       await addToLog(LogPath, Group, Result);

//       Result ? TestResults.Passed++ : TestResults.Failed++;
//       console.log(" ");
//       console.log(" ");
//     } catch (error) {
//       console.error("Error processing group:", error);
//     }
//   };
