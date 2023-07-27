pub mod transfer_directories {
    use crate::functions::file_fisher::file_fisher::file_fisher;

    pub async fn transfer_directories(
        dirty_directories: &mut Vec<String>,
        dirty_directory: &str,
    ) -> Vec<String> {
        let mut transferred_directories: Vec<String> = Vec::new();
        // let mut completed_directories: u32 = 0;

        for dir in dirty_directories.iter_mut() {
            println!("Currently transferring: {}", dir);

            let file_details_res = file_fisher(&(dirty_directory.to_owned() + dir)).await;
            if let Ok(file_details) = file_details_res {
                println!("File Details: {}", file_details.len());

                for detail in file_details {
                    println!("item: {}", detail);
                    transferred_directories.push(detail);
                }
            } else {
                eprintln!("Error reading directory: {:?}", file_details_res.err());
            }

            // completed_directories += 1;
        }
        transferred_directories
    }

    // const transferDirectories = async (DirtyDirectories, DirtyDirectory) => {
    //     const TransferredDirectories = [];
    //     let completedDirectories = 0;

    //     for (const Dir of DirtyDirectories) {
    //       console.log("Currently transferring:", Dir);
    //       try {
    //         // Get files from all directories in each top level directory
    //         const AllFileDetails = await FileFisher(`${DirtyDirectory}/${Dir}`);
    //         console.log("All File Details:", AllFileDetails.length);

    //         AllFileDetails.forEach((FileDetails) => {
    //           // console.log("FD:", FileDetails);
    //           TransferredDirectories.push(FileDetails);
    //         });
    //         completedDirectories++;
    //       } catch (error) {
    //         console.error("Error transferring directory from top level directory:", error);
    //       }
    //     }

    //     return TransferredDirectories;
    //   };
}
