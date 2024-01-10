use std::fs;
use std::io::Write;
use std::path::Path;

pub trait JSONExporter {
    fn export(&mut self, s: String);
}

pub struct ConsoleExporter {}

impl JSONExporter for ConsoleExporter {
    fn export(&mut self, s: String) {
        print!("{}", s)
    }
}

pub struct FileExporter {
    dir: String,
}

impl FileExporter {
    pub fn new(dir: String) -> FileExporter {
        if !Path::new(&dir).exists() {
            fs::create_dir_all(&dir).unwrap();
        }

        FileExporter {
            dir,
        }
    }
}

impl JSONExporter for FileExporter {
    fn export(&mut self, s: String) {
        let current_timestamp = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let file_name = format!("{}/{}.json", self.dir, current_timestamp);
        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }
}