use synth_library::data_builders::miscellaneous::generic_string_list_factory::GenericStringListFactory;
use synth_library::data_builders::network::ip_factory::IpFactory;
use synth_library::data_builders::utils::my_rand::MyRand;

use crate::data_helpers::live_data::objects::record_object::RecordObject;
use crate::data_helpers::live_data::objects::resolve_object::ResolveObject;
use crate::data_helpers::string_values::StringValues;
use crate::synth_library::data_builders::datetime::time_factory::TimeFactory;
use crate::synth_library::data_builders::identifiers::uuid_factory::UuidFactory;

pub struct DataRecordBuilder<'a> {
    domain_name_list: GenericStringListFactory<'a>,
    top_level_domain_name_list: GenericStringListFactory<'a>,
    file_type_list: GenericStringListFactory<'a>,
    rule_list: GenericStringListFactory<'a>,
    tag_list: GenericStringListFactory<'a>,
    family_list: GenericStringListFactory<'a>,
    sandbox_list: GenericStringListFactory<'a>,
    source_url_list: GenericStringListFactory<'a>,
    record_type_list: GenericStringListFactory<'a>,
    values_list: GenericStringListFactory<'a>,
}

/// # About
/// Builds an object with the collection of values from the StringValues object.
/// Contains a static lifetime reference "'a" because we are pulling from StringValues
/// as references, instead of cloning values. The lifetime of the objects pulled should
/// live until the record is turned into a String value for exporting.
/// 
/// We tag each GenericStringListFactory object with a name and a value. The name is
/// for tracking the status of the GenericStringListFactory object.
impl<'a> DataRecordBuilder<'a> {
    pub fn new(string_values: &'a StringValues) -> DataRecordBuilder<'a> {
        DataRecordBuilder {
            domain_name_list: GenericStringListFactory::new(
                String::from("domain names"),
                &string_values.names_list,
            ),
            top_level_domain_name_list: GenericStringListFactory::new(
                String::from("top level domain values"),
                &string_values.top_level_domain_name_list,
            ),
            file_type_list: GenericStringListFactory::new(
                String::from("file types"),
                &string_values.file_type_list,
            ),
            rule_list: GenericStringListFactory::new(
                String::from("rules"),
                &string_values.rule_list,
            ),
            tag_list: GenericStringListFactory::new(
                String::from("tags"),
                &string_values.tag_list,
            ),
            family_list: GenericStringListFactory::new(
                String::from("family values"),
                &string_values.family_list,
            ),
            sandbox_list: GenericStringListFactory::new(
                String::from("Sandbox values"),
                &string_values.sandbox_list,
            ),
            source_url_list: GenericStringListFactory::new(
                String::from("Source URL values"),
                &string_values.names_list,
            ),
            record_type_list: GenericStringListFactory::new(
                String::from("record types"),
                &string_values.record_type_list,
            ),
            values_list: GenericStringListFactory::new(
                String::from("values list"),
                &string_values.numeric_values_string_list,
            ),
        }
    }

    // Builds a Record for the FileDataFactory object and passes it back
    pub fn build_record(&self) -> RecordObject {
        const _MAX_DOMAIN_NAMES: usize = 5;
        const _MAX_TAGS: usize = 2;
        const _MAX_RESOLV_VALUES: usize = 3;
        const _MAX_SOURCE_URL: usize = 5;

        let _temp_record = RecordObject::new(
            TimeFactory::get_current_time_rfc3339(),
            self.domain_name_list
                .get_multiple_string_values(1, _MAX_DOMAIN_NAMES),
            self.top_level_domain_name_list.get_random_string_value(),
            self.file_type_list.get_random_string_value(),
            self.rule_list.get_random_string_value(),
            self.tag_list.get_multiple_string_values(0, _MAX_TAGS),
            self.family_list.get_random_string_value(),
            self.sandbox_list.get_random_string_value(),
            self.source_url_list
                .get_multiple_string_values(0, _MAX_SOURCE_URL),
            UuidFactory::get_uuid(),
            Self::get_resolve_objects(_MAX_RESOLV_VALUES),
            self.record_type_list.get_random_string_value(),
            self.values_list.get_random_string_value(),
            TimeFactory::get_current_time_rfc3339(),
        );

        _temp_record
    }

    fn get_resolve_objects(max_resolve_values: usize) -> Vec<ResolveObject> {
        let mut temp_vec: Vec<ResolveObject> = Vec::new();
        let string_values = StringValues::new();
        let t_list =
            GenericStringListFactory::new(String::from("loc"), &string_values.tag_list);
        let v_list = GenericStringListFactory::new(
            String::from("values"),
            &string_values.numeric_values_string_list,
        );
        for _x in 0..MyRand::get_usize_rand_between(0, max_resolve_values) {
            let resolve_object = ResolveObject::new(
                t_list.get_random_string_value(),
                IpFactory::get_random_public_ip(),
                v_list.get_random_string_value(),
            );
            temp_vec.push(resolve_object);
        }
        temp_vec
    }

    pub fn wrap_records_to_string(records: Vec<RecordObject>) -> String {
        let mut _results = String::new();
        for record in records {
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
    pub fn test_data_record_builder() {
        let string_values = StringValues::new();
        let data_record_builder = DataRecordBuilder::new(&string_values);
        let record_object = data_record_builder.build_record();
        dbg!(record_object);
    }
}
