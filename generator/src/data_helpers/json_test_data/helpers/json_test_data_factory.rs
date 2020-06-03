use crate::data_helpers::json_test_data::helpers::{
    data_object_test_record_builder::DataObjectTestRecordBuilder,
    test_file_data_factory::TestFileDataFactory,
    test_port_data_factory::TestPortDataFactory,
};

use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::string_values::StringValues;

use log::{debug, info};

pub struct JsonTestDataFactory {
    argument_helper: ArgumentHelper,
}

impl JsonTestDataFactory {
    pub fn new(argument_helper: ArgumentHelper) -> JsonTestDataFactory {
        debug!("new");
        info!("Building new JsonTestDataFactory");
        JsonTestDataFactory { argument_helper }
    }

    /// # About
    /// Starts the JsonTestDataFactory with the supplied ArgumentHelper values
    pub fn start(&self) {
        debug!("start");

        let string_values = StringValues::new();
        let data_record_builder = DataObjectTestRecordBuilder::new(&string_values);
        
        if ArgumentHelper::OUTPUT_SWITCH_TO_FILE == self.argument_helper.output_format {
            info!("Starting output as file stream");
            TestFileDataFactory::stream_file_data(data_record_builder, &self.argument_helper);
        } 
        else if ArgumentHelper::OUTPUT_SWITCH_TO_PORT == self.argument_helper.output_format
        {
            info!("Starting output as port stream");
            TestPortDataFactory::stream_port_data(data_record_builder, &self.argument_helper);
        }
    }
}
