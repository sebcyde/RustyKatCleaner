pub mod file_fisher {
    use futures::future::{BoxFuture, FutureExt};
    use std::io;
    use tokio::fs;

    pub fn file_fisher(directory: &str) -> BoxFuture<'static, Result<Vec<String>, io::Error>> {
        let future: BoxFuture<'static, Result<Vec<String>, io::Error>> = async move {
            let mut files: Vec<String> = Vec::new();
            if let Ok(mut entries) = fs::read_dir(directory).await {
                while let Some(entry) = entries.next_entry().await? {
                    let path: String = entry.path().to_str().unwrap().to_string();
                    if entry.file_type().await?.is_dir() {
                        let sub_files = file_fisher(&path).await?;
                        files.extend(sub_files);
                    } else {
                        files.push(path);
                    }
                }
            }

            Ok(files)
        }
        .boxed();

        future
    }
}
