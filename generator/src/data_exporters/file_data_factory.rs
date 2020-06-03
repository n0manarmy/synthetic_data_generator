extern crate rand;

use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::live_data::{
    helpers::data_record_builder::DataRecordBuilder,
    objects::record_object::RecordObject,
};
use crate::synth_library::data_exporters::file_io_helper::FileIOHelper;

type RecordsHolder = Vec<RecordObject>;

pub struct FileDataFactory {}

impl FileDataFactory {

    /// Generates data to be exported to a FileIOHelper function, writing to file.
    pub fn stream_file_data(data_record_builder: DataRecordBuilder, arguments: ArgumentHelper) {
        // records counter
        let mut records_counter: usize = 0;
        
        // container for the records we build
        let mut records_holder: RecordsHolder = Vec::new();

        // thread collector. Finish this function with for thread in threads to join
        // and finish out runs.
        let mut threads = vec![];

        // Loop and count the records generated
        for _ in 0..arguments.count {
            records_holder.push(data_record_builder.build_record());
            records_counter = records_counter + 1;

            // If the buffer threshold is met then spawn a thread 
            // to the FileIOHelper to write the results to file and reset
            if records_holder.len() % arguments.output_buffer == 0 {

                let r_counter = records_counter.clone();
                let path = arguments.output_name.clone();
                
                threads.push(std::thread::spawn(move || {
                    FileIOHelper::write_string(
                        path,
                        r_counter, 
                        DataRecordBuilder::wrap_records_to_string(records_holder));
                }));
                records_holder = Vec::new();
            }
        }

        threads.push(std::thread::spawn(move || {
            FileIOHelper::write_string(
                arguments.output_name,
                records_counter.clone(),
                DataRecordBuilder::wrap_records_to_string(records_holder));
        }));

        for thread in threads {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]

mod test {

    use super::*;
    use crate::data_helpers::string_values::StringValues;

    #[test]
    fn test_stream_file_data() {
        let string_values = StringValues::new();
        let argument_helper = ArgumentHelper::new();
        let data_record_builder = DataRecordBuilder::new(&string_values);

        FileDataFactory::stream_file_data(data_record_builder, argument_helper);
    }
}
