pub mod mod_host_file {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::io::{self, prelude::*, BufReader};
    use serde_json::Value::String;

    const DEFAULT_HOST_FILE_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

    pub fn open_host_file() -> io::Result<File> {
        let f = File::open(DEFAULT_HOST_FILE_PATH)?;
        Ok(f)
    }

    pub fn get_host_file_bufreader() -> BufReader<File> {
        let file_result = open_host_file();
        match file_result {
            Ok(val) => {
                return BufReader::new(val);
            },
            Err(err) => {
                panic!("Failed to create bufreader! {}", err);
            }
        }
    }

    pub fn save_content_host_file(content: &str) -> io::Result<()> {
        let mut f = File::options().write(true).truncate(true).open(DEFAULT_HOST_FILE_PATH)?;
        f.write_all(content.to_string().as_bytes())?;
        f.sync_data()?;
        Ok(())
    }
}