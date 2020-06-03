use rand::Rng;

#[warn(dead_code)]
#[derive(Debug)]
pub struct MyRand {
    pub get_rand_number: u32,
}

impl MyRand {
    pub fn get_u32_rand_between(min: u32, max: u32) -> u32 {
        rand::thread_rng().gen_range(min, max + 1)
    }

    pub fn get_usize_rand_between(min: usize, max: usize) -> usize {
        rand::thread_rng().gen_range(min, max + 1)
    }

    pub fn get_true_false() -> bool {
        if rand::thread_rng().gen_range(0, 2) == 0 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_builders::utils::my_rand::MyRand;

    #[test]
    fn test_get_rand_number() {
        let min_value = 1;
        let max_value = 1001;
        for _num in 1..100000 {
            let value = MyRand::get_u32_rand_between(min_value, max_value);
            assert!(value >= min_value);
            assert!(value <= max_value);
        }
    }
}
