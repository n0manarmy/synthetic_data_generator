use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct JsonTestObject {
    string_object: &'static str,
    int_object: usize,
    boolean_object: bool,
}

impl JsonTestObject {
    pub const STRING_OBJECT: &'static str = "NESTED_STRING_OBJECT";
    pub const INT_OBJECT: &'static str = "NESTED_INT_OBJECT";
    pub const BOOLEAN_OBJECT: &'static str = "NESTED_BOOLEAN_OBJECT";

    pub fn new(
        string_object: &'static str,
        int_object: usize,
        boolean_object: bool,
    ) -> JsonTestObject {
        JsonTestObject {
            string_object,
            int_object,
            boolean_object,
        }
    }
}

impl Serialize for JsonTestObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("json_test_object", 3)?;
        state.serialize_field(Self::STRING_OBJECT, &self.string_object)?;
        state.serialize_field(Self::INT_OBJECT, &self.int_object)?;
        state.serialize_field(Self::BOOLEAN_OBJECT, &self.boolean_object)?;
        state.end()
    }
}
