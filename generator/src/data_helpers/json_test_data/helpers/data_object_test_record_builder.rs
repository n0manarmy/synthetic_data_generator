use synth_library::data_builders::miscellaneous::generic_string_list_factory::GenericStringListFactory;
use synth_library::data_builders::utils::my_rand::MyRand;
use log::{debug};

use crate::data_helpers::json_test_data::objects::data_object_test_record::DataObjectTestRecord;
use crate::data_helpers::json_test_data::objects::json_test_object::JsonTestObject;

use crate::data_helpers::string_values::StringValues;

pub struct DataObjectTestRecordBuilder<'a> {
    string_object: GenericStringListFactory<'a>,
    array_string_object: GenericStringListFactory<'a>,
}

impl<'a> DataObjectTestRecordBuilder<'a> {
    /// # About
    /// Builds an object with the collection of values from the StringValues object.
    /// Contains a static lifetime reference "'a" because we are pulling from StringValues
    /// as references, instead of cloning values. The lifetime of the objects pulled should
    /// live until the record is turned into a String value for exporting.
    /// We tag each GenericStringListFactory object with a name and a value. The name is
    /// for tracking the status of the GenericStringListFactory object.
    pub fn new(string_values: &'a StringValues) -> DataObjectTestRecordBuilder<'a> {
        debug!("new");
        DataObjectTestRecordBuilder {
            string_object: GenericStringListFactory::new(
                String::from("string_object"),
                &string_values.names_list,
            ),
            array_string_object: GenericStringListFactory::new(
                String::from("domain names"),
                &string_values.names_list,
            ),
        }
    }

    /// Builds a Record for the FileDataFactory object and passes it back
    pub fn build_record(&self) -> DataObjectTestRecord {
        let _temp_record = DataObjectTestRecord::new(
            self.string_object.get_random_string_value(),
            MyRand::get_usize_rand_between(0, 100),
            self.array_string_object.get_multiple_string_values(0, 5),
            Self::get_json_test_object(5),
            MyRand::get_true_false(),
            "null",
        );
        _temp_record
    }

    fn get_json_test_object(max_resolve_values: usize) -> Vec<JsonTestObject> {
        let mut temp_vec: Vec<JsonTestObject> = Vec::new();
        let string_values = StringValues::new();
        let t_list = GenericStringListFactory::new(
            String::from("STRING_VALUES"),
            &string_values.tag_list,
        );
        for _x in 0..MyRand::get_usize_rand_between(0, max_resolve_values) {
            let resolve_object = JsonTestObject::new(
                t_list.get_random_string_value(),
                MyRand::get_usize_rand_between(0, 100),
                MyRand::get_true_false(),
            );
            temp_vec.push(resolve_object);
        }
        temp_vec
    }

    pub fn wrap_records_to_string(records: Vec<DataObjectTestRecord>) -> String {
        let mut _results = String::new();
        for record in records {
            // _results.push_str(&record.get_formatted_record());
            _results.push_str(&record.get_serde_formatted_json());
            _results.push_str("\n");
        }
        _results
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    pub fn test_data_object_test_record_builder() {
        let string_values = StringValues::new();
        let data_record_builder = DataObjectTestRecordBuilder::new(&string_values);
        let record_object = data_record_builder.build_record();
        dbg!(record_object);
    }
}
