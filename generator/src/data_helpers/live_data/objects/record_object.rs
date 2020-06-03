use crate::data_helpers::live_data::objects::resolve_object::ResolveObject;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct RecordObject {
    date_time_start: String,
    domain_names: Vec<&'static str>,
    top_level_domain: &'static str,
    file_type: &'static str,
    rule: &'static str,
    tags: Vec<&'static str>,
    family: &'static str,
    sandbox: &'static str,
    source_url: Vec<&'static str>,
    sid: String,
    resolv_list: Vec<ResolveObject>,
    record_type: &'static str,
    value: &'static str,
    date_time_end: String,
}

impl RecordObject {
    const _TIME_STAMP_START: &'static str = "TIME_STAMP";
    const _DOMAIN_NAME_LABEL: &'static str = "DOMAIN";
    const _TOP_LEVEL_DOMAIN_LABEL: &'static str = "TOP_LEVEL_DOMAIN";
    const _FILE_TYPE_LABEL: &'static str = "FILE_TYPE";
    const _RULES_LABEL: &'static str = "RULE";
    const _TAG_LABEL: &'static str = "TAG";
    const _FAMILY_LABEL: &'static str = "FAMILY";
    const _SANDBOX_LABEL: &'static str = "SANDBOX";
    const _SOURCE_LABEL: &'static str = "SOURCE";
    const _SID_LABEL: &'static str = "SID";
    const _RESOLV_LABEL: &'static str = "RESOLV";
    const _RECORD_TYPE_LABEL: &'static str = "RECORD_TYPE";
    const _VALUE_LABEL: &'static str = "VALUE";
    const _TIME_END_LABEL: &'static str = "TIME_END";

    pub fn new(
        date_time_start: String,
        domain_names: Vec<&'static str>,
        top_level_domain: &'static str,
        file_type: &'static str,
        rule: &'static str,
        tags: Vec<&'static str>,
        family: &'static str,
        sandbox: &'static str,
        source_url: Vec<&'static str>,
        sid: String,
        resolv_list: Vec<ResolveObject>,
        record_type: &'static str,
        value: &'static str,
        date_time_end: String,
    ) -> RecordObject {
        RecordObject {
            date_time_start,
            domain_names,
            top_level_domain,
            file_type,
            rule,
            tags,
            family,
            sandbox,
            source_url,
            sid,
            resolv_list,
            record_type,
            value,
            date_time_end,
        }
    }

    fn concat_domain_names(domain_names: &Vec<&'static str>) -> String {
        let domain_iter = domain_names.iter();
        let mut domain_str = String::new();
        for d in domain_iter {
            domain_str.push_str(d);
        }
        domain_str
    }

    pub fn get_serde_formatted_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Serialize for RecordObject {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("record_object", 13)?;
        state.serialize_field(Self::_TIME_STAMP_START, &self.date_time_start)?;
        state.serialize_field(Self::_DOMAIN_NAME_LABEL, &Self::concat_domain_names(&self.domain_names))?;
        state.serialize_field(Self::_TOP_LEVEL_DOMAIN_LABEL, &self.top_level_domain)?;
        state.serialize_field(Self::_FILE_TYPE_LABEL, &self.file_type)?;
        state.serialize_field(Self::_RULES_LABEL, &self.rule)?;
        state.serialize_field(Self::_TAG_LABEL, &self.tags)?;
        state.serialize_field(Self::_FAMILY_LABEL, &self.family)?;
        state.serialize_field(Self::_SANDBOX_LABEL, &self.sandbox)?;
        state.serialize_field(Self::_SOURCE_LABEL, &self.source_url)?;
        state.serialize_field(Self::_SID_LABEL, &self.sid)?;
        state.serialize_field(Self::_RESOLV_LABEL, &self.resolv_list)?;
        state.serialize_field(Self::_RECORD_TYPE_LABEL, &self.record_type)?;
        state.serialize_field(Self::_VALUE_LABEL, &self.value)?;
        state.serialize_field(Self::_TIME_END_LABEL, &self.date_time_end)?;
        state.end()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    pub fn test_get_formatted_live_record_object() {
        let results = String::from("{\"TIME_STAMP\":\"date_time\",\"DOMAIN\":\"domain_name1domain_name2\",\"TOP_LEVEL_DOMAIN\":\"tld\",\"FILE_TYPE\":\"FILE_TYPE\",\"RULE\":\"RULE\",\"TAG\":[\"tag1\"],\"FAMILY\":\"FAMILY\",\"SANDBOX\":\"SANDBOX\",\"SOURCE\":[\"src1\",\"src2\"],\"SID\":\"SID\",\"RESOLV\":[{\"geo\":\"loc1\",\"ip\":\"ip1\",\"asn\":\"asn1\"},{\"geo\":\"loc2\",\"ip\":\"ip2\",\"asn\":\"asn2\"}],\"RECORD_TYPE\":\"RECORD_TYPE\",\"VALUE\":\"VALUE\",\"TIME_END\":\"TIME_END\"}");
        let record_object = RecordObject::new(
            "date_time".to_string(),
            vec!["domain_name1", "domain_name2"],
            "tld",
            "FILE_TYPE",
            "RULE",
            vec!["tag1"],
            "FAMILY",
            "SANDBOX",
            vec!["src1", "src2"],
            "SID".to_string(),
            vec![
                ResolveObject::new("loc1", "ip1".to_string(), "asn1"),
                ResolveObject::new("loc2", "ip2".to_string(), "asn2"),
            ],
            "RECORD_TYPE",
            "VALUE",
            "TIME_END".to_string(),
        );
        // dbg!(record_object.get_formatted_record());
        assert_eq!(results, record_object.get_serde_formatted_json());
    }
}
