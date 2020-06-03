pub struct JsonFormatter {}

impl JsonFormatter {
    pub const STATE_NEW: &'static str = "NEW";
    pub const STATE_END: &'static str = "END";
    pub const STATE_CONT: &'static str = "CONT";

    const _END: &'static str = "}\n";
    const _QOUTES: &'static str = "\"";
    const _COLON: &'static str = ":";
    const _COMMA: &'static str = ",";
    const _OPEN_BRACKET: &'static str = "[";
    const _CLOSE_BRACKET: &'static str = "]";
    const _OPEN_BRACE: &'static str = "{";
    const _CLOSE_BRACE: &'static str = "}";

    /// Takes a tuple containing a pointer to its name and a vector of strings
    /// and concats them to a single string.
    ///
    /// # Example
    /// ```
    /// ("List of Strings", vec!["One", "Two", "Three", "Four"]) = "OneTwoThreeFour"
    /// ```
    /// The state of the overall Json formatted string is required too.
    ///
    /// # Example
    /// If this value exists in the middle of the process to build a json object:
    /// ```
    /// format_vector_to_strings_concat(JsonFormatter::STATE_CONT, (tuple_name, tuple_value))
    ///
    pub fn format_vector_to_strings_concat(value: Vec<&'static str>) -> String {
        let mut collected_values = String::new();
        for _x in value {
            collected_values.push_str(&_x);
        }
        // Self::format_string(state, (label, &collected_values))
        collected_values
    }

    pub fn format_null(state: &str, (label, value): (&str, &'static str)) -> String {
        match state {
            Self::STATE_NEW => {
                return [
                    Self::_OPEN_BRACE,
                    &Self::format_null_builder(label, value.to_string()),
                ]
                .concat();
            }
            Self::STATE_CONT => {
                return [
                    Self::_COMMA,
                    &Self::format_null_builder(label, value.to_string()),
                ]
                .concat();
            }
            Self::STATE_END => {
                return [
                    Self::_COMMA,
                    &Self::format_null_builder(label, value.to_string()),
                    Self::_END,
                ]
                .concat();
            }
            _ => {
                panic!("I should not be here!");
            }
        }
    }

    pub fn format_boolean(state: &str, (label, value): (&str, bool)) -> String {
        match state {
            Self::STATE_NEW => {
                return [
                    Self::_OPEN_BRACE,
                    &Self::format_boolean_builder(label, value.to_string()),
                ]
                .concat();
            }
            Self::STATE_CONT => {
                return [
                    Self::_COMMA,
                    &Self::format_boolean_builder(label, value.to_string()),
                ]
                .concat();
            }
            Self::STATE_END => {
                return [
                    Self::_COMMA,
                    &Self::format_boolean_builder(label, value.to_string()),
                    Self::_END,
                ]
                .concat();
            }
            _ => {
                panic!("I should not be here!");
            }
        }
    }

    pub fn format_int(state: &str, (label, value): (&str, usize)) -> String {
        match state {
            Self::STATE_NEW => {
                return [Self::_OPEN_BRACE, &Self::format_int_builder(label, value)].concat();
            }
            Self::STATE_CONT => {
                return [Self::_COMMA, &Self::format_int_builder(label, value)].concat();
            }
            Self::STATE_END => {
                return [
                    Self::_COMMA,
                    &Self::format_int_builder(label, value),
                    Self::_END,
                ]
                .concat();
            }
            _ => {
                panic!("I should not be here!");
            }
        }
    }

    pub fn format_string(state: &str, (label, value): (&str, String)) -> String {
        match state {
            Self::STATE_NEW => {
                return [
                    Self::_OPEN_BRACE,
                    &Self::format_string_builder(label, value.to_string()),
                ]
                .concat();
            }
            Self::STATE_CONT => {
                return [
                    Self::_COMMA,
                    &Self::format_string_builder(label, value.to_string()),
                ]
                .concat();
            }
            Self::STATE_END => {
                return [
                    Self::_COMMA,
                    &Self::format_string_builder(label, value.to_string()),
                    Self::_END,
                ]
                .concat();
            }
            _ => {
                panic!("I should not be here!");
            }
        }
    }

    pub fn format_vector_to_json_array(
        _state: &str,
        (label, value): (&str, Vec<&'static str>),
    ) -> String {
        let mut collected_values = String::new();
        // Match for empty Vector which would be an empty JSON array
        if value.len() == 0 {
            if _state == Self::STATE_CONT || _state == Self::STATE_END {
                return [
                    Self::_COMMA,
                    Self::_QOUTES,
                    label,
                    Self::_QOUTES,
                    Self::_COLON,
                    Self::_OPEN_BRACKET,
                    Self::_CLOSE_BRACKET,
                ]
                .concat();
            } else {
                return [
                    Self::_QOUTES,
                    label,
                    Self::_QOUTES,
                    Self::_COLON,
                    Self::_OPEN_BRACKET,
                    Self::_CLOSE_BRACKET,
                ]
                .concat();
            }
        }
        // Proceed with normal parsing of JSON array values
        else {
            if _state == Self::STATE_CONT || _state == Self::STATE_END {
                collected_values.push_str(Self::_COMMA);
            }

            collected_values.push_str(
                &[
                    Self::_QOUTES,
                    label,
                    Self::_QOUTES,
                    Self::_COLON,
                    Self::_OPEN_BRACKET,
                ]
                .concat(),
            );

            for _pos in 0..value.len() {
                if value.len() == 1 {
                    // dbg!("value.len() == 1");
                    collected_values.push_str(
                        &[
                            &Self::format_array_builder(&value[_pos]),
                            Self::_CLOSE_BRACKET,
                        ]
                        .concat(),
                    );
                    return collected_values;
                } else if _pos == 0 {
                    collected_values.push_str(&Self::format_array_builder(&value[_pos]));
                } else if _pos == (value.len() - 1) {
                    // dbg!("else if");
                    collected_values.push_str(&Self::format_array(
                        Self::STATE_END,
                        (label, value[_pos].to_string()),
                    ));
                } else {
                    // dbg!("else");
                    collected_values.push_str(&Self::format_array(
                        Self::STATE_CONT,
                        (label, value[_pos].to_string()),
                    ));
                }
            }
            collected_values
        }
    }

    //Format based on criteria provided above
    fn format_array(state: &str, (label, value): (&str, String)) -> String {
        match state {
            Self::STATE_NEW => {
                return [
                    Self::_QOUTES,
                    label,
                    Self::_QOUTES,
                    Self::_COLON,
                    Self::_OPEN_BRACKET,
                    &Self::format_array_builder(&value),
                ]
                .concat();
            }
            Self::STATE_CONT => {
                return [Self::_COMMA, &Self::format_array_builder(&value)].concat();
            }
            Self::STATE_END => {
                return [
                    Self::_COMMA,
                    &Self::format_array_builder(&value),
                    Self::_CLOSE_BRACKET,
                ]
                .concat();
            }
            _ => {
                panic!("I should not be here!");
            }
        }
    }

    fn format_array_builder(value: &str) -> String {
        [Self::_QOUTES, &value, Self::_QOUTES].concat()
    }

    fn format_vector_to_string(value: &Vec<(String, String)>) -> String {
        let mut _temp = String::new();
        if value.len() == 0 {
            return [Self::_OPEN_BRACKET, Self::_CLOSE_BRACKET].concat();
        }
        for x in 0..value.len() {
            if x == 0 {
                _temp.push_str(
                    &[
                        Self::_QOUTES,
                        &value[x].0,
                        Self::_QOUTES,
                        Self::_COLON,
                        Self::_QOUTES,
                        &value[x].1,
                        Self::_QOUTES,
                    ]
                    .concat(),
                );
            } else {
                _temp.push_str(
                    &[
                        Self::_COMMA,
                        Self::_QOUTES,
                        &value[x].0,
                        Self::_QOUTES,
                        Self::_COLON,
                        Self::_QOUTES,
                        &value[x].1,
                        Self::_QOUTES,
                    ]
                    .concat(),
                );
            }
        }
        _temp
    }

    fn format_int_builder(label: &str, value: usize) -> String {
        [
            Self::_QOUTES,
            label,
            Self::_QOUTES,
            Self::_COLON,
            &value.to_string(),
        ]
        .concat()
    }

    fn format_boolean_builder(label: &str, value: String) -> String {
        [Self::_QOUTES, label, Self::_QOUTES, Self::_COLON, &value].concat()
    }

    fn format_null_builder(label: &str, value: String) -> String {
        [Self::_QOUTES, label, Self::_QOUTES, Self::_COLON, &value].concat()
    }

    fn format_string_builder(label: &str, value: String) -> String {
        [
            Self::_QOUTES,
            label,
            Self::_QOUTES,
            Self::_COLON,
            Self::_QOUTES,
            &value,
            Self::_QOUTES,
        ]
        .concat()
    }

    pub fn format_json_multidimensional_array(
        state: &str,
        (label, value): (&str, Vec<Vec<(String, String)>>),
    ) -> String {
        let mut _temp = String::new();
        if Self::STATE_NEW == state {
            _temp = [
                Self::_QOUTES,
                label,
                Self::_QOUTES,
                Self::_COLON,
                Self::_OPEN_BRACKET,
            ]
            .concat();
            for _x in 0..value.len() {
                if _x == (value.len() - 1) {
                    _temp.push_str(
                        &[
                            Self::_OPEN_BRACE,
                            &Self::format_vector_to_string(&value[_x]),
                            Self::_CLOSE_BRACE,
                        ]
                        .concat(),
                    );
                } else {
                    _temp.push_str(
                        &[
                            Self::_OPEN_BRACE,
                            &Self::format_vector_to_string(&value[_x]),
                            Self::_CLOSE_BRACE,
                            Self::_COMMA,
                        ]
                        .concat(),
                    );
                }
            }
            _temp.push_str(&[Self::_CLOSE_BRACKET].concat());
        } else {
            _temp = [
                Self::_COMMA,
                Self::_QOUTES,
                label,
                Self::_QOUTES,
                Self::_COLON,
                Self::_OPEN_BRACKET,
            ]
            .concat();
            for _x in 0..value.len() {
                if _x == (value.len() - 1) {
                    _temp.push_str(
                        &[
                            Self::_OPEN_BRACE,
                            &Self::format_vector_to_string(&value[_x]),
                            Self::_CLOSE_BRACE,
                        ]
                        .concat(),
                    );
                } else {
                    _temp.push_str(
                        &[
                            Self::_OPEN_BRACE,
                            &Self::format_vector_to_string(&value[_x]),
                            Self::_CLOSE_BRACE,
                            Self::_COMMA,
                        ]
                        .concat(),
                    );
                }
            }
            _temp.push_str(&[Self::_CLOSE_BRACKET].concat());
        }
        _temp
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    pub fn test_format_vector_to_string() {
        const _LOCATION_LABEL: &'static str = "geo";
        const _IP_LABEL: &'static str = "ip";
        const _ASN_LABEL: &'static str = "asn";
        let values = vec![
            (_LOCATION_LABEL.to_string(), "label1".to_string()),
            (_IP_LABEL.to_string(), "127.0.0.1".to_string()),
            (_ASN_LABEL.to_string(), "asn1".to_string()),
        ];
        // let values = vec![one, two, three];

        let results = String::from("\"geo\":\"label1\",\"ip\":\"127.0.0.1\",\"asn\":\"asn1\"");
        dbg!(JsonFormatter::format_vector_to_string(&values));
        assert_eq!(results, JsonFormatter::format_vector_to_string(&values));
    }

    #[test]
    pub fn test_format_json_multidimensional_array() {
        const _LOCATION_LABEL: &'static str = "geo";
        const _IP_LABEL: &'static str = "ip";
        const _ASN_LABEL: &'static str = "asn";
        let one = vec![
            (_LOCATION_LABEL.to_string(), "label1".to_string()),
            (_IP_LABEL.to_string(), "127.0.0.1".to_string()),
            (_ASN_LABEL.to_string(), "asn1".to_string()),
        ];
        let two = vec![
            (_LOCATION_LABEL.to_string(), "label2".to_string()),
            (_IP_LABEL.to_string(), "127.0.0.2".to_string()),
            (_ASN_LABEL.to_string(), "asn2".to_string()),
        ];
        let three = vec![
            (_LOCATION_LABEL.to_string(), "label3".to_string()),
            (_IP_LABEL.to_string(), "127.0.0.3".to_string()),
            (_ASN_LABEL.to_string(), "asn3".to_string()),
        ];
        let values = ("resolv", vec![one, two, three]);
        // let values = vec![one, two, three];

        let results = "\"resolv\":[{\"geo\":\"label1\",\"ip\":\"127.0.0.1\",\"asn\":\"asn1\"},{\"geo\":\"label2\",\"ip\":\"127.0.0.2\",\"asn\":\"asn2\"},{\"geo\":\"label3\",\"ip\":\"127.0.0.3\",\"asn\":\"asn3\"}]";
        dbg!(JsonFormatter::format_json_multidimensional_array(
            JsonFormatter::STATE_NEW,
            values.clone()
        ));
        assert_eq!(
            results,
            JsonFormatter::format_json_multidimensional_array(
                JsonFormatter::STATE_NEW,
                values.clone()
            )
        );
    }

    #[test]
    pub fn test_format_vector_to_strings_concat() {
        let results = String::from(",\"test value\":\"OneTwoThreeFour\"");
        // let test_value = ("test value", vec!["One", "Two", "Three", "Four"]);
        let test_value = vec!["One", "Two", "Three", "Four"];
        // dbg!(JsonFormatter::format_vector_to_strings_concat(JsonFormatter::STATE_CONT, test_value));
        assert_eq!(
            results,
            JsonFormatter::format_vector_to_strings_concat(test_value)
        );
    }

    #[test]
    pub fn test_format_vector_to_string_array_empty() {
        let _empty_results_end = String::from(",\"test_empty\":[]");
        let _empty_results_new = String::from("\"test_empty\":[]");
        let _empty_results_cont = String::from(",\"test_empty\":[]");
        let empty_test = ("test_empty", vec![]);

        assert_eq!(
            _empty_results_new,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_NEW,
                empty_test.clone()
            )
        );
        assert_eq!(
            _empty_results_end,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_END,
                empty_test.clone()
            )
        );
        assert_eq!(
            _empty_results_cont,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_CONT,
                empty_test.clone()
            )
        );
    }

    #[test]
    pub fn test_format_vector_to_string_array_values() {
        let _values_results_new =
            String::from("\"test_values\":[\"One\",\"Two\",\"Three\",\"Four\"]");
        let _values_results_cont =
            String::from(",\"test_values\":[\"One\",\"Two\",\"Three\",\"Four\"]");
        let _values_results_end =
            String::from(",\"test_values\":[\"One\",\"Two\",\"Three\",\"Four\"]");
        let _value_results_new = String::from("\"test_values\":[\"One\"]");
        let _value_results_cont = String::from(",\"test_values\":[\"One\"]");
        let _value_results_end = String::from(",\"test_values\":[\"One\"]");
        let values_test = ("test_values", vec!["One", "Two", "Three", "Four"]);
        let value_test = ("test_values", vec!["One"]);

        assert_eq!(
            _values_results_new,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_NEW,
                values_test.clone()
            )
        );
        assert_eq!(
            _values_results_cont,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_CONT,
                values_test.clone()
            )
        );
        assert_eq!(
            _values_results_end,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_END,
                values_test.clone()
            )
        );
        assert_eq!(
            _value_results_new,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_NEW,
                value_test.clone()
            )
        );
        assert_eq!(
            _value_results_cont,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_CONT,
                value_test.clone()
            )
        );
        assert_eq!(
            _value_results_end,
            JsonFormatter::format_vector_to_json_array(
                JsonFormatter::STATE_END,
                value_test.clone()
            )
        );
    }

    #[test]
    pub fn test_format_string() {
        // let test_value = ("test value", vec!["One", "Two", "Three", "Four"]);
        let test_value = vec!["One", "Two", "Three", "Four"];
        dbg!(JsonFormatter::format_vector_to_strings_concat(test_value));
    }
}
