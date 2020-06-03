use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct FileIOHelper {}

impl FileIOHelper {
    // pub fn new(output_name: &str) -> FileIOHelper {
    //     let mut path_buf = PathBuf::new();
    //     path_buf.push(output_name);

    //     FileIOHelper { path_buf }
    // }

    pub fn write_string(output_name: String, count: usize, _output: String) {

        let mut path_buf = PathBuf::new();
        path_buf.push(output_name);
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path_buf)
        {
            Err(why) => panic!(
                "Couldn't create {}: {}",
                path_buf.display(),
                why.to_string()
            ),
            Ok(file) => file,
        };

        match file.write_all(_output.as_bytes()) {
            Err(why) => {
                panic!(
                    "Couldn't write to {}: {}",
                    path_buf.display(),
                    why.to_string()
                );
            }
            Ok(_) => println!("Wrote line {} to file {}", count, path_buf.display()),
        }
    }

    pub fn write_vector(_count: usize, _output: Vec<String>, output_name: &str) {
        let path = Path::new(output_name);
        let display = path.display();

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path)
        {
            Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
            Ok(file) => file,
        };

        for record in _output {
            match file.write_all(record.as_bytes()) {
                Err(why) => {
                    eprintln!("Couldn't write to {}: {}", display, why);
                }
                Ok(_) => println!("Wrote line {} to file {}", _count, display),
            }
        }
    }
}
