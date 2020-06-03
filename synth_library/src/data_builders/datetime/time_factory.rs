use chrono::{TimeZone, Utc};

pub struct TimeFactory {
    start_time: i64,
}

impl TimeFactory {
    pub fn new_from_epoch(_start_time: i64) -> TimeFactory {
        TimeFactory {
            start_time: _start_time,
        }
    }

    pub fn get_current_time_rfc3339() -> String {
        Utc::now().to_rfc3339()
    }

    pub fn add_to_and_get_date_time_from_start_time(
        &mut self,
        secs: i64,
        nano_secs: u32,
    ) -> String {
        self.start_time = self.start_time + secs;
        let dt = Utc.timestamp(self.start_time, nano_secs);
        dt.to_rfc3339()
    }
}

#[cfg(test)]
mod tests {

    use crate::data_builders::datetime::time_factory::TimeFactory;
    use crate::data_builders::utils::my_rand::MyRand;

    #[test]
    pub fn test_get_current_date_from_epoch() {
        let mut time_factory = TimeFactory::new_from_epoch(0);
        println!(
            "{:?}",
            time_factory.add_to_and_get_date_time_from_start_time(100_000_000, 0)
        );
    }

    #[test]
    pub fn test_incrementing_date() {
        let mut time_factory = TimeFactory::new_from_epoch(0);
        for _x in 1..1000 {
            let adjust = MyRand::get_u32_rand_between(100_000, 1_000_000);
            println!(
                "{:?}",
                time_factory.add_to_and_get_date_time_from_start_time(adjust as i64, 0)
            );
        }
    }

    #[test]
    pub fn test_time_stamp_factory() {
        dbg!(TimeFactory::get_current_time_rfc3339());
    }

    #[test]
    pub fn test_get_now() {
        dbg!(TimeFactory::get_current_time_rfc3339());
        println!("{}", TimeFactory::get_current_time_rfc3339());
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("{}", TimeFactory::get_current_time_rfc3339());
    }
}
