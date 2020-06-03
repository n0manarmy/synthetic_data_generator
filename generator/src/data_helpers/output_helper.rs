pub struct OutputHelper {}

impl OutputHelper {

    pub fn print_vec_string(values: Vec<String>) {
        for _x in &values {
            print!("{}, ", _x);
        }
        println!();
    }

    pub fn print_string(string: &String) {
        println!("{}", string);
    }
}