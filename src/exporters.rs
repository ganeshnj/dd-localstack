use std::fs;
use std::io::Write;
use std::path::Path;

pub trait StringExporter {
    fn export(&mut self, s: String);
}

pub struct ConsoleStringExporter {}

impl StringExporter for ConsoleStringExporter {
    fn export(&mut self, s: String) {
        print!("{}", s)
    }
}

pub struct FileStringExporter {
    dir: String,
}

impl FileStringExporter {
    pub fn new(dir: String) -> FileStringExporter {
        if !Path::new(&dir).exists() {
            fs::create_dir_all(&dir).unwrap();
        }

        FileStringExporter {
            dir,
        }
    }
}

impl StringExporter for FileStringExporter {
    fn export(&mut self, s: String) {
        // get highest file number
        let mut file_counter = 0;
        for entry in fs::read_dir(&self.dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_number = file_name.split(".").next().unwrap().parse::<u64>().unwrap();
            if file_number > file_counter {
                file_counter = file_number;
            }
        }
        let file_name = format!("{}/{}.json", self.dir, file_counter + 1);
        let mut file = fs::File::create(file_name).unwrap();
        file.write_all(s.as_bytes()).unwrap();
    }
}