pub mod create_excel {}
use std::error::Error;
use std::path::Path;

use excel::write::{Workbook, Worksheet, Xlsx};

pub async fn create_excel(directory: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("Creating Excel Workbook");
    let mut workbook = Workbook::create(Path::new(directory))?;

    println!("Created Excel Workbook");
    println!("Adding Worksheet");
    println!(" ");

    // Create the worksheet
    let mut worksheet = workbook.create_worksheet(None)?;

    println!("Added Excel Worksheet {}", file_name);

    // Save the worksheet to the specified output path
    let excel_path = format!("{}.xlsx", directory);
    let xlsx = Xlsx::from(&workbook);
    xlsx.save(&excel_path)?;

    println!("Workbook saved to: {}", excel_path);
    println!(" ");

    Ok(())
}
