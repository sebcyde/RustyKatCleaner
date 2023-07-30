pub mod file_data_reader {
    use csv::ReaderBuilder;
    use std::error::Error;
    use tokio::fs;

    pub async fn file_data_reader(
        file_path: &str,
    ) -> Result<Option<Vec<Vec<String>>>, Box<dyn Error>> {
        println!("Reading file: {}", file_path);
        let mut data = Vec::new();

        let input = fs::read(file_path).await?;
        let mut reader = ReaderBuilder::new()
            .comment(Some(b'#'))
            .from_reader(input.as_slice());

        for result in reader.records() {
            let record = result?;
            data.push(record.iter().map(|field| field.to_string()).collect());
        }

        println!("Finished reading file: {}", file_path);
        Ok(Some(data))
    }
}
