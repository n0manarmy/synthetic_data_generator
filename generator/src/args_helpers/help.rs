use super::argument_helper::ArgumentHelper;

pub struct Help;

impl Help {
    /// # About
    /// Prints the help file to std out
    pub fn print_help() {
        let help = format!(
        "
        
        Synthetic Data Generator

        Usage:
        --help, -?\t\t\t\t--  This output.

        {} (debug|info|error|warn)\t--  Sets the logging level for the application.
                                                    This defaults to info.
        
        {} (boolean)\t\t--  Generates a data formatted file containing all
                                                    JSON data values for testing purposes.
                                                    (defaults to false).

        {} [file|port]\t\t--  The output direction for the data. 
                                                    file will output to file name specified in --output-name.
                                                    port will output to a port. Specify the port with the --port switch.
                                                    (defaults to file).

        {} (name)\t\t\t--  Filename of output data file.
                                                    (defaults to pre_generated_name.json).

        {} (number)\t\t\t--  Number of records to generate for file OR
                                                    Number of records per second for port. Counting for port is done in
                                                    half second increments. A minimum value of 2 is required.
                                                    (defaults to 100).
                                            
        {} (number)\t\t--  Buffer size to use when generating and then output records with the
                                                    file output format. Usage was tested around 25000 to 50000.
                                                    (defaults to 10000).
        
        {} (json)\t\t\t--  [NOT IMPLEMENTED YET] Format to present data to the output destination. Support to be added
                                                    for XML and CSV.
                                                    (defaults to json).
        
        -- Below only necessary when streaming to ports

        {} (number)\t\t\t--  The port number to output to.
                                                    (defaults to 4000).

        {} [variable|fixed]\t--  Fixed rate attempts to output as close to the count as possible.
                                                    Variable rate will vary the amount to appear closer to actual traffic.
                                                    (defaults to fixed).

        {} (ip address)\t\t--  Source IP address to connect to for port streaming data.
                                                    (defaults to 127.0.01)
                                                    
        {} (number)\t\t\t--  Source port to connect to and stream from.
                                                    (defaults to 40504)
                                                    \n",
        ArgumentHelper::LOG_LEVEL_SWITCH,
        ArgumentHelper::VALIDATE_JSON_SWITCH,
        ArgumentHelper::OUTPUT_DST_SWITCH,
        ArgumentHelper::OUTPUT_NAME_SWITCH,
        ArgumentHelper::COUNT_SWITCH,
        ArgumentHelper::OUTPUT_BUFFER_SIZE_SWITCH,
        ArgumentHelper::OUTPUT_FORMAT_SWITCH,
        ArgumentHelper::DST_PORT_SWITCH,
        ArgumentHelper::DST_PORT_RATE_SWITCH,
        ArgumentHelper::SRC_IP_SWITCH,
        ArgumentHelper::SRC_PORT_SWITCH);
        println!("{}", help);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn init_help() {
        dbg!(Help::print_help());
    }
}
