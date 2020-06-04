use crate::data_builders::utils::my_rand::MyRand;
use log::{debug};

#[derive(Debug)]
pub struct GenericStringListFactory<'a> {
    string_values: &'a [&'static str],
    list_name: String,
}

impl<'a> GenericStringListFactory<'a> {
    pub fn new(list_name: String, string_values: &'a [&'static str]) -> GenericStringListFactory<'a> {
        debug!("new");
        GenericStringListFactory {
            string_values,
            list_name,
        }
    }

    pub fn get_multiple_string_values(
        &self,
        min_count: usize,
        max_count: usize,
    ) -> Vec<&'static str> {
        let mut slice: Vec<&'static str> = Vec::new();
        let quantity = MyRand::get_usize_rand_between(min_count, max_count);

        if min_count > self.string_values.len() {
            return slice;
        } else {
            for _x in 0..quantity {
                slice.push(
                    self.string_values
                        [MyRand::get_usize_rand_between(0, self.string_values.len() - 1)],
                );
            }
        }
        slice
    }

    pub fn get_random_string_value(&self) -> &'static str {
        self.string_values[MyRand::get_usize_rand_between(0, self.string_values.len() - 1)]
    }

    pub fn get_string_value_at_pos(&self, pos: usize) -> &str {
        self.string_values[pos]
    }

    pub fn get_list_info(&self) -> String {
        let _new_line = "\n";
        let _list_name_label = "list name: ";
        let _list_size_label = "list size: ";
        let _list_info = &[
            _list_name_label,
            &self.list_name.clone(),
            _new_line,
            _list_size_label,
            &self.string_values.len().to_string(),
        ]
        .concat();
        _list_info.to_string()
    }
}

#[cfg(test)]
mod tests {

    use crate::data_builders::miscellaneous::generic_string_list_factory::GenericStringListFactory;

    #[test]
    pub fn test_get_default_types() {
        let list = vec!["zip", "doc", "exe", "sh", "com"];
        let generic_string_list_factory =
            GenericStringListFactory::new("generic list".to_string(), &list);
        dbg!(generic_string_list_factory.get_random_string_value());
        dbg!(generic_string_list_factory.get_string_value_at_pos(3));
        dbg!(generic_string_list_factory.string_values);
    }

    #[test]
    pub fn get_get_multiple_random_string_values() {
        let list = vec!["zip", "doc", "exe", "sh", "com"];
        let generic_string_list_factory =
            GenericStringListFactory::new("generic list".to_string(), &list);
        dbg!(generic_string_list_factory.get_multiple_string_values(1, 3));
        let results = generic_string_list_factory.get_multiple_string_values(50, 90);
        assert!(results.len() == 0);
        dbg!(results);
    }
}
