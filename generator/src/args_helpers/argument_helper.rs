//! # Argument Helper
//! Collection of expected switches for the command line arguments.
#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub struct ArgumentHelper {
    pub log_level: LogLevel,
    pub count: usize,
    pub output_format: String,
    pub dst_port: String,
    pub output_name: String,
    pub port_rate: String,
    pub output_buffer: usize,
    pub output_data_type: String,
    pub src_port: String,
    pub src_ip_address: String,
    pub help: bool,
    pub validate_json: bool,
}

/// Sets default values for arguments if none received.
///
/// # Default Config
/// * log_level: info,
/// * count: 10000,
/// * output: String::from("file"),
/// * dst_port: String::from("4000"),
/// * output_name: String::from("pre_generated_name.json"),
/// * port_rate: String::from("fixed"),
/// * output_buffer: 10000,
/// * help: false,
/// * output_data_type: String::from("json"),
/// * src_port: String::from("40504"),
/// * src_ip_address: String::from("127.0.0.1"),
/// * validate_json: false,
///
impl ArgumentHelper {
    pub const OUTPUT_DST_SWITCH: &'static str = "--output-dst";
    pub const OUTPUT_NAME_SWITCH: &'static str = "--output-name";
    pub const COUNT_SWITCH: &'static str = "--count";
    pub const OUTPUT_BUFFER_SIZE_SWITCH: &'static str = "--output-buffer-size";
    pub const DST_PORT_SWITCH: &'static str = "--dst-port";
    pub const DST_PORT_RATE_SWITCH: &'static str = "--dst-port-rate";
    pub const OUTPUT_FORMAT_SWITCH: &'static str = "--output-format";
    pub const SRC_PORT_SWITCH: &'static str = "--src-port";
    pub const SRC_IP_SWITCH: &'static str = "--src-ip-address";
    pub const HELP_SWITCHES: [&'static str; 2] = ["--help", "-?"];
    pub const VALIDATE_JSON_SWITCH: &'static str = "--validate-json";
    pub const LOG_LEVEL_SWITCH: &'static str = "--log-level";
    pub const PORT_RATE_FIXED: &'static str = "fixed";
    pub const PORT_RATE_VARIABLE: &'static str = "variable";
    pub const OUTPUT_SWITCH_TO_FILE: &'static str = "file";
    pub const OUTPUT_SWITCH_TO_PORT: &'static str = "port";

    /// # About
    /// Constructor with default values, set to minimal values for running.
    /// 
    pub fn new() -> ArgumentHelper {
        ArgumentHelper {
            log_level: LogLevel::Info,
            count: 10000,
            output_format: String::from("file"),
            dst_port: String::from("4000"),
            output_name: String::from("pre_generated_name.json"),
            port_rate: String::from("fixed"),
            output_buffer: 10000,
            help: false,
            output_data_type: String::from("json"),
            src_port: String::from("40504"),
            src_ip_address: String::from("127.0.0.1"),
            validate_json: false,
        }
    }

    /// Parses the supplied debug level to enum LogLevel
    /// 
    fn parse_log_level(val: &str) -> LogLevel {
        match val {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }

    /// Match the arguments from the command line and parse them into
    /// an ArgumentHelper object. At this time no validation of the
    /// expected value is being checked.
    /// 
    pub fn parse_args(mut self, args: Vec<String>) -> ArgumentHelper {
        for _x in 0..args.len() {
            match args[_x].as_ref() {
                _ if args[_x] == Self::HELP_SWITCHES[0] => {
                    self.help = true;
                }
                _ if args[_x] == Self::HELP_SWITCHES[1] => {
                    self.help = true;
                }
                Self::LOG_LEVEL_SWITCH => {
                    self.log_level = Self::parse_log_level(&args[_x + 1]);
                }
                Self::SRC_PORT_SWITCH => {
                    self.src_port = args[_x + 1].clone();
                }
                Self::SRC_IP_SWITCH => {
                    self.src_ip_address = args[_x + 1].clone();
                }
                Self::OUTPUT_FORMAT_SWITCH => {
                    self.output_format = args[_x + 1].clone();
                }
                Self::COUNT_SWITCH => {
                    self.count = args[_x + 1].clone().parse::<usize>().unwrap();
                }
                Self::OUTPUT_DST_SWITCH => {
                    self.output_format = args[_x + 1].to_string();
                }
                Self::DST_PORT_SWITCH => {
                    self.dst_port = args[_x + 1].clone();
                }
                Self::OUTPUT_NAME_SWITCH => {
                    self.output_name = args[_x + 1].clone();
                }
                Self::DST_PORT_RATE_SWITCH => {
                    self.port_rate = args[_x + 1].clone();
                }
                Self::OUTPUT_BUFFER_SIZE_SWITCH => {
                    self.output_buffer = args[_x + 1].clone().parse::<usize>().unwrap();
                }
                Self::VALIDATE_JSON_SWITCH => {
                    self.validate_json = args[_x + 1].clone().parse::<bool>().unwrap();
                }
                &_ => {}
            }
        }
        self
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_init_argument_helper() {
        let argument_helper = ArgumentHelper::new();
        assert_eq!(argument_helper.output_buffer, 10000);
        assert_eq!(argument_helper.output_data_type, "file");
        assert_eq!(argument_helper.dst_port, "4000");
        assert_eq!(argument_helper.output_name, "pre_generated_name.json");
        assert_eq!(argument_helper.port_rate, "fixed");
        assert_eq!(argument_helper.output_buffer, 10000);
        assert_eq!(argument_helper.help, false);
        // assert_eq!(argument_helper.get_output_data_type(), "json");
        assert_eq!(argument_helper.src_port, "40504");
        assert_eq!(argument_helper.src_ip_address, "127.0.0.1");
        assert_eq!(argument_helper.validate_json, false);
        dbg!(argument_helper);
    }
}
