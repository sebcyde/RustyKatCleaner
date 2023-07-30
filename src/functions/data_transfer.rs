pub mod data_transfer {
    use excel::DataType;
    use excel::Fill;
    use excel::Workbook;
    use excel::Worksheet;
    use std::error::Error;

    use crate::functions::file_data_reader::file_data_reader::file_data_reader;

    const USER_NAME: &str = "Sebastian Cyde";

    pub async fn data_transfer(
        file_path: &str,
        section: &str,
        workbook: &mut Workbook,
        worksheet: &mut Worksheet,
        excel_path: &str,
    ) -> Result<bool, Box<dyn Error>> {
        println!("Starting data transfer");

        let current_date = chrono::Utc::today().format("%d/%m/%Y").to_string();
        let file_data = file_data_reader(file_path).await.unwrap();

        if let Some(file_data) = file_data {
            println!("Populating Excel Worksheet");

            for (i, data_object) in file_data.iter().enumerate() {
                println!("Writing Record: {}", i + 1);
                worksheet.set_column(i, i, 30.0, None);

                worksheet.write_string(i + 2, 0, &data_object[0], None);
                worksheet.write_string(i + 2, 1, &data_object[4], None);
                worksheet.write_string(i + 2, 2, &data_object[5], None);
                worksheet.write_string(i + 2, 3, &data_object[6], None);
                worksheet.write_string(i + 2, 4, &data_object[7], None);

                let result_cell = worksheet.get_cell_mut(i + 2, 4).unwrap();

                match &data_object[7] {
                    "PASSED" => {
                        let fill = Fill::new(
                            DataType::Pattern,
                            Some("solid"),
                            Some("c1fba4"),
                            None,
                            None,
                            None,
                        );
                        result_cell.set_fill(fill);
                    }
                    "FAILED" => {
                        let fill = Fill::new(
                            DataType::Pattern,
                            Some("solid"),
                            Some("bc4749"),
                            None,
                            None,
                            None,
                        );
                        result_cell.set_fill(fill);
                    }
                    _ => {}
                }
            }

            // Signoff - user signatures etc
            println!("\nSigning Sheet");
            worksheet.write_string(0, 0, &format!("Signed By: {}", USER_NAME), None);
            worksheet.write_string(0, 1, &format!("Creation Date: {}", current_date), None);
            worksheet.write_string(0, 2, &format!("Test Section: {}", section), None);

            println!("\nData transfer complete");
            println!("Saving...");

            // Save the workbook to the specified output path
            workbook.close().unwrap();
            excel::write_file(excel_path, workbook)?;

            let result = file_data
                .iter()
                .filter(|test| test[7] != "Status" && !test[7].is_empty())
                .all(|test| test[7] == "PASSED");

            println!("Result: {}", if result { "PASS" } else { "FAIL" });
            Ok(result)
        } else {
            eprintln!("Failed to read HTML data");
            Err("Failed to read HTML data".into())
        }
    }
}
