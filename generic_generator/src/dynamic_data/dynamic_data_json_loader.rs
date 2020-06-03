use std::fs;
use std::env;
// use json::{JsonValue, JsonError};
use serde_json;

pub struct DynamicDataJsonLoader {

}

impl DynamicDataJsonLoader {

    pub fn read_file(file_name: &'static str) -> String {
        let contents = fs::read_to_string(file_name).expect("File not loaded and read");
        // match json::parse(&contents) {
        //     Ok(json) => json,
        //     Err(why) => panic!(why),
        // }
        // serde_json::to_string(&contents).unwrap()
        // serde_json::to_string_pretty(&contents).unwrap()
        // return results;
        match serde_json::to_string(&contents) {
            Ok(val) => val,
            Err(why) => panic!(why),
        }
    }

    // pub fn extract_file(json_value: JsonValue) {
    //     dbg!(json_value);
    //     // let instantiated = object!{}
    // }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    pub fn test_load_json_file() {
        let _file_name = "test_files/test_data_desc.json";
        let contents = DynamicDataJsonLoader::read_file(_file_name);
        println!("{}", contents);
    }
}