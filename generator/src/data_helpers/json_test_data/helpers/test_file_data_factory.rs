extern crate rand;

use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::json_test_data::helpers::data_object_test_record_builder::DataObjectTestRecordBuilder;
use crate::data_helpers::json_test_data::objects::data_object_test_record::DataObjectTestRecord;
use crate::synth_library::data_exporters::file_io_helper::FileIOHelper;

use std::path::{Path, PathBuf};

pub struct TestFileDataFactory {}

impl TestFileDataFactory {
    /// # About
    /// Generates data to be exported to a FileIOHelper function, writing to file.
    pub fn stream_file_data(
        data_record_builder: DataObjectTestRecordBuilder,
        arguments: &ArgumentHelper,
    ) {
        let mut records_generated: usize = 0;
        let mut records: Vec<DataObjectTestRecord> = Vec::new();

        // let file_io_helper = FileIOHelper::new(&arguments.output_name);
        // let mut path = PathBuf::new();
        // path.push(&arguments.output_name);

        // Loop and count the records generated
        for _x in 0..arguments.count {
            records.push(data_record_builder.build_record());
            records_generated = records_generated + 1;

            // If the bugger is met then pass records to the FileIOHelper and reset
            if records.len() % arguments.output_buffer == 0 {
                FileIOHelper::write_string(
                    arguments.output_name.clone(),
                    records_generated.clone(),
                    DataObjectTestRecordBuilder::wrap_records_to_string(records),
                );
                records = Vec::new();
            }
        }
        FileIOHelper::write_string(
            arguments.output_name.clone(),
            records_generated.clone(),
            DataObjectTestRecordBuilder::wrap_records_to_string(records),
        );
    }
}

#[cfg(test)]

mod test {

    use super::*;
    use crate::data_helpers::string_values::StringValues;

    #[test]
    fn test_file_data_factory_stream_file_data() {
        let string_values = StringValues::new();
        let argument_helper = ArgumentHelper::new();
        let data_record_builder = DataObjectTestRecordBuilder::new(&string_values);

        TestFileDataFactory::stream_file_data(data_record_builder, &argument_helper);
    }
}
