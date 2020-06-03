use crate::data_exporters::file_data_factory::FileDataFactory;
use crate::data_exporters::port_data_factory::PortDataFactory;

use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::live_data::helpers::data_record_builder::DataRecordBuilder;
use crate::data_helpers::string_values::StringValues;

pub struct LiveDataFactory {
    argument_helper: ArgumentHelper,
}

impl LiveDataFactory {
    pub fn new(argument_helper: ArgumentHelper) -> LiveDataFactory {
        LiveDataFactory { argument_helper }
    }

    /// # About
    /// Starts the LiveDataFactory with the supplied ArgumentHelper values
    pub fn start(self) {
        // Container for all the string values from the library used to generate data
        let string_values = StringValues::new();

        // Data object that is the basis for the synthetic data generated
        let data_record_builder = DataRecordBuilder::new(&string_values);
        if ArgumentHelper::OUTPUT_SWITCH_TO_FILE == self.argument_helper.output_format {
            println!("Output as file stream");
            FileDataFactory::stream_file_data(data_record_builder, self.argument_helper);
        } 
        else if ArgumentHelper::OUTPUT_SWITCH_TO_PORT == self.argument_helper.output_format
        {
            println!("Output as port stream");
            PortDataFactory::stream_port_data(data_record_builder, &self.argument_helper);
        } else {
            panic!("Path error, no output destination found");
        }
    }
}
