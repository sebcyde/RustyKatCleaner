pub mod group_files {
    use std::{collections::HashSet, path::Path};

    pub struct GroupedFiles {
        pub pdf: Option<String>,
        pub html: Option<String>,
        pub csv: Option<String>,
        pub section: Option<String>,
    }

    pub async fn group_files(files: Vec<String>) -> Vec<GroupedFiles> {
        let mut base_names: HashSet<String> = std::collections::HashSet::new();
        let mut grouped_files: Vec<GroupedFiles> = Vec::new();

        for file in &files {
            if file.contains(".csv") {
                base_names.insert(
                    Path::new(file)
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }

        for base_name in base_names {
            let grouped_files_for_base_name: Vec<String> = files
                .iter()
                .filter(|file| Path::new(file).file_stem().unwrap().to_string_lossy() == base_name)
                .cloned() // Clone the Strings
                .collect();

            let pdf = grouped_files_for_base_name
                .iter()
                .find(|file| file.contains(".pdf"))
                .cloned();
            let html = grouped_files_for_base_name
                .iter()
                .find(|file| file.contains(".html"))
                .cloned();
            let csv = grouped_files_for_base_name
                .iter()
                .find(|file| file.contains(".csv"))
                .cloned();
            let section = pdf.clone().and_then(|pdf_file| {
                pdf_file
                    .split("\\")
                    .nth(11)
                    .map(|section| section.to_string())
            });
            let grouped_file: GroupedFiles = GroupedFiles {
                pdf,
                html,
                csv,
                section,
            };

            grouped_files.push(grouped_file);
        }

        grouped_files
    }
}
