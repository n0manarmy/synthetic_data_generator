use uuid::Uuid;

#[allow(dead_code)]
pub struct UuidFactory;

impl UuidFactory {
    pub fn get_uuid() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }
}

#[cfg(test)]

mod tests {

    use crate::data_builders::identifiers::uuid_factory::UuidFactory;

    #[test]
    pub fn test_get_random_uuid() {
        for _x in 1..100 {
            // println!("{}", uuid_factory.get_uuid());
            println!("{}", UuidFactory::get_uuid());
        }
    }
}
