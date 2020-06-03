use crate::data_helpers::json_test_data::objects::json_test_object::JsonTestObject;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct DataObjectTestRecord {
    string_object: &'static str,
    int_object: usize,
    array_string_object: Vec<&'static str>,
    nested_array_string_object: Vec<JsonTestObject>,
    boolean_object: bool,
    null_object: &'static str,
}

impl DataObjectTestRecord {
    pub const STRING_OBJECT: &'static str = "STRING_OBJECT";
    pub const INT_OBJECT: &'static str = "INT_OBJECT";
    pub const ARRAY_STRING_OBJECT: &'static str = "ARRAY_STRING_OBJECT";
    pub const NESTED_ARRAY_STRING_OJBECT: &'static str = "NESTED_ARRAY_STRING_OBJECT";
    pub const BOOLEAN_OBJECT: &'static str = "BOOLEAN_OBJECT";
    pub const NULL_OBJECT: &'static str = "NULL_OBJECT";

    pub fn new(
        string_object: &'static str,
        int_object: usize,
        array_string_object: Vec<&'static str>,
        nested_array_string_object: Vec<JsonTestObject>,
        boolean_object: bool,
        null_object: &'static str,
    ) -> DataObjectTestRecord {
        DataObjectTestRecord {
            string_object,
            int_object,
            array_string_object,
            nested_array_string_object,
            boolean_object,
            null_object,
        }
    }

    pub fn get_serde_formatted_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Serialize for DataObjectTestRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("json_test_data", 6)?;
        state.serialize_field(Self::STRING_OBJECT, &self.string_object)?;
        state.serialize_field(Self::INT_OBJECT, &self.int_object)?;
        state.serialize_field(Self::ARRAY_STRING_OBJECT, &self.array_string_object)?;
        state.serialize_field(
            Self::NESTED_ARRAY_STRING_OJBECT,
            &self.nested_array_string_object,
        )?;
        state.serialize_field(Self::BOOLEAN_OBJECT, &self.boolean_object)?;
        state.serialize_field(Self::NULL_OBJECT, &self.null_object)?;
        state.end()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    pub fn test_get_formatted_data_object_test_record() {
        let results = String::from("{\"STRING_OBJECT\":\"date_time\",\"INT_OBJECT\":1,\"ARRAY_STRING_OBJECT\":[\"src1\",\"src2\"],\"NESTED_ARRAY_STRING_OBJECT\":[{\"NESTED_STRING_OBJECT\":\"loc1\",\"NESTED_INT_OBJECT\":1,\"NESTED_BOOLEAN_OBJECT\":true},{\"NESTED_STRING_OBJECT\":\"loc2\",\"NESTED_INT_OBJECT\":2,\"NESTED_BOOLEAN_OBJECT\":false}],\"BOOLEAN_OBJECT\":true,\"NULL_OBJECT\":\"null\"}");
        let record_object = DataObjectTestRecord::new(
            "date_time",
            1,
            vec!["src1", "src2"],
            vec![
                JsonTestObject::new("loc1", 1, true),
                JsonTestObject::new("loc2", 2, false),
            ],
            true,
            "null",
        );
        // dbg!(record_object.get_formatted_record());
        assert_eq!(results, record_object.get_serde_formatted_json());
    }
}
