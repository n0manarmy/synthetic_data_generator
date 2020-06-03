use crate::data_builders::utils::my_rand::MyRand;

pub struct AsnFactory {}

impl AsnFactory {
    const ASN_16_MIN: usize = 1;
    const ASN_16_MAX: usize = 65535;
    const ASN_32_MIN: usize = 65536;
    const ASN_32_MAX: usize = 4294967295;

    pub fn new() -> AsnFactory {
        AsnFactory {}
    }

    pub fn get_non_unique_16_bit_asn(&self) -> usize {
        MyRand::get_usize_rand_between(Self::ASN_16_MIN, Self::ASN_16_MAX)
    }

    pub fn get_non_unique_32_bit_asn(&self) -> usize {
        MyRand::get_usize_rand_between(Self::ASN_32_MIN, Self::ASN_32_MAX)
    }
}

#[cfg(test)]

mod tests {
    use crate::data_builders::asn::asn_factory::AsnFactory;

    #[test]
    fn test_get_16_bit_asn() {
        let asn_factory = AsnFactory::new();
        dbg!(asn_factory.get_non_unique_16_bit_asn());
    }

    #[test]
    fn test_get_32_bit_asn() {
        let asn_factory = AsnFactory::new();
        dbg!(asn_factory.get_non_unique_32_bit_asn());
    }
}
