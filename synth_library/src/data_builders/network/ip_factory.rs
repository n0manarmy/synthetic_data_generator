use crate::data_builders::utils::my_rand::MyRand;

#[derive(Debug)]
pub struct IpFactory;

impl IpFactory {
    const PRIVATE_CLASS_A: u8 = 10;
    const PRIVATE_CLASS_B: u8 = 172;
    const PRIVATE_CLASS_C: u8 = 192;
    const PRIVATE_CLASS_C_SECOND_OCTET: u8 = 168;
    const PRIVATE_LOCAL: u8 = 127;
    const ONE: u8 = 1;
    const ZERO: u8 = 0;

    const NON_PUBLIC_IP_VALUES: [u8; 4] = [
        Self::PRIVATE_CLASS_A,
        Self::PRIVATE_CLASS_B,
        Self::PRIVATE_CLASS_C,
        Self::PRIVATE_LOCAL,
    ];

    /**
     * Public accessible library functions
     */
    pub fn get_random_private_ip() -> String {
        let first_oct = Self::NON_PUBLIC_IP_VALUES
            [MyRand::get_usize_rand_between(0, Self::NON_PUBLIC_IP_VALUES.len() - 1)];
        let second_oct = Self::get_random_octet_number();
        let third_oct = Self::get_random_octet_number();
        let fourth_oct = Self::get_random_octet_number();

        Self::format_ip_address_to_string(first_oct, second_oct, third_oct, fourth_oct)
    }

    pub fn get_class_ip(class_type: u8) -> String {
        let mut first_oct: u8 = 0;
        let mut second_oct: u8 = 0;
        let mut third_oct: u8 = 0;
        let mut fourth_oct: u8 = 0;

        match class_type {
            Self::PRIVATE_CLASS_A => {
                first_oct = Self::PRIVATE_CLASS_A;
                second_oct = Self::get_random_octet_number();
                third_oct = Self::get_random_octet_number();
                fourth_oct = Self::get_random_octet_number();
            }
            Self::PRIVATE_CLASS_B => {
                first_oct = Self::PRIVATE_CLASS_B;
                second_oct = MyRand::get_u32_rand_between(16, 31) as u8;
                third_oct = Self::get_random_octet_number();
                fourth_oct = Self::get_random_octet_number();
            }
            Self::PRIVATE_CLASS_C => {
                first_oct = Self::PRIVATE_CLASS_C;
                second_oct = Self::PRIVATE_CLASS_C_SECOND_OCTET;
                third_oct = Self::get_random_octet_number();
                fourth_oct = Self::get_random_octet_number();
            }
            Self::PRIVATE_LOCAL => {
                first_oct = Self::PRIVATE_LOCAL;
                second_oct = Self::ZERO;
                third_oct = Self::ZERO;
                fourth_oct = Self::ONE;
            }
            _ => {}
        }
        Self::format_ip_address_to_string(first_oct, second_oct, third_oct, fourth_oct)
    }

    pub fn get_random_public_ip() -> String {
        let first_oct: u8 = Self::get_ip_does_not_contain(&Self::NON_PUBLIC_IP_VALUES);
        let second_oct = MyRand::get_u32_rand_between(1, 255) as u8;
        let third_oct = MyRand::get_u32_rand_between(1, 255) as u8;
        let fourth_oct = MyRand::get_u32_rand_between(1, 255) as u8;

        Self::format_ip_address_to_string(first_oct, second_oct, third_oct, fourth_oct)
    }

    /**
     * private helper functions for above public library functions
     */
    fn format_ip_address_to_string(
        first_oct: u8,
        second_oct: u8,
        third_oct: u8,
        fourth_oct: u8,
    ) -> String {
        let _dot = String::from(".");
        return format!(
            "{}{}{}{}{}{}{}",
            first_oct, _dot, second_oct, _dot, third_oct, _dot, fourth_oct
        );
    }

    fn get_random_octet_number() -> u8 {
        MyRand::get_u32_rand_between(0, 255) as u8
    }

    fn get_ip_does_not_contain(values: &[u8]) -> u8 {
        loop {
            let value = MyRand::get_u32_rand_between(1, 254) as u8;
            if !values.contains(&value) {
                return value;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::data_builders::network::ip_factory::IpFactory;

    #[test]
    pub fn test_get_random_ip() {
        dbg!(IpFactory::get_random_private_ip());
    }

    #[test]
    pub fn test_get_class_ips() {
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_A));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_A));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_A));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_A));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_A));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_B));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_B));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_B));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_B));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_B));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_C));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_C));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_C));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_C));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_CLASS_C));
        dbg!(IpFactory::get_class_ip(IpFactory::PRIVATE_LOCAL));
    }

    #[test]
    pub fn test_get_random_public_ip() {
        dbg!(IpFactory::get_random_public_ip());
    }
}
