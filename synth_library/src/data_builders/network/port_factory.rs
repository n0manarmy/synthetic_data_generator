use crate::data_builders::utils::my_rand::MyRand;

pub struct PortFactory {}

impl PortFactory {
    const MINIMUM_PORT: u32 = 0;
    const RESERVED_PORT_MAX: u32 = 0;
    const PUBLIC_PORT_MIN: u32 = 1025;
    const PUBLIC_PORT_MAX: u32 = 65535;

    pub fn get_reserved_port() -> u32 {
        MyRand::get_u32_rand_between(Self::MINIMUM_PORT, Self::RESERVED_PORT_MAX)
    }

    pub fn get_public_port() -> u32 {
        MyRand::get_u32_rand_between(Self::PUBLIC_PORT_MIN, Self::PUBLIC_PORT_MAX)
    }
}
