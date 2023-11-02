use std::fs;
use std::io::Write;
use std::path::Path;
use crate::datadog_span::DatadogSpan;

pub(crate) trait Exporter {
    fn export(&mut self, spans: Vec<Vec<DatadogSpan>>);
}

pub(crate) struct ConsoleExporter {}

impl Exporter for ConsoleExporter {
    fn export(&mut self, spans: Vec<Vec<DatadogSpan>>) {
        let beautified = serde_json::to_string_pretty(&spans).unwrap();
        print!("{}", beautified)
    }
}

static mut FILE_COUNTER: u64 = 0;

pub(crate) struct FileExporter {
    dir: String,
}

impl FileExporter {
    pub(crate) fn new(dir: String) -> FileExporter {
        if !Path::new(&dir).exists() {
            fs::create_dir_all(&dir).unwrap();
        }

        FileExporter {
            dir,
        }
    }
}

impl Exporter for FileExporter {
    fn export(&mut self, spans: Vec<Vec<DatadogSpan>>) {
        unsafe { FILE_COUNTER += 1; }
        let file_name = format!("{}/{}.json", self.dir, unsafe { FILE_COUNTER });
        let mut file = fs::File::create(file_name).unwrap();
        let json = serde_json::to_string_pretty(&spans).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

