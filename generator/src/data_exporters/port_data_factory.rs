extern crate rand;

use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::live_data::helpers::data_record_builder::DataRecordBuilder;
use crate::synth_library::data_exporters::port_streamer::PortStreamer;

use rand::Rng;

pub struct PortDataFactory {}

impl PortDataFactory {
    /// Builds the port stream, determining rate and fixed or variable
    /// then proceeds with call to start_stream.
    ///
    pub fn stream_port_data(data_record_builder: DataRecordBuilder, arguments: &ArgumentHelper) {
        let rate: usize;
        let port_rate_fixed: bool;

        let port_streamer = PortStreamer::new(
            &arguments.src_ip_address,
            &arguments.src_port,
            &arguments.dst_port,
        );

        if ArgumentHelper::PORT_RATE_FIXED == &arguments.port_rate {
            rate = arguments.count / 2;
            port_rate_fixed = true;
        } else {
            rate = Self::get_rand_rate(arguments.count);
            port_rate_fixed = false;
            if rate <= 0 {
                panic!("_rate <= 0, no count to stream to port");
            }
        }

        Self::start_stream(
            arguments.count,
            port_rate_fixed,
            rate,
            data_record_builder,
            port_streamer,
        );
    }

    /// Loops indefinitely, changing rates if variable every half second,
    /// based on 20% swing on original rate.
    ///
    fn start_stream(
        messages_per_second: usize,
        port_rate_fixed: bool,
        mut rate: usize,
        data_record_builder: DataRecordBuilder,
        port_streamer: PortStreamer,
    ) {
        let _half_second = std::time::Duration::new(0, 500000000);

        loop {
            std::thread::sleep(_half_second);

            println!(
                "{} records generated and streamed to {}",
                rate.clone(),
                port_streamer.get_dst()
            );
            if rate <= 0 {
                panic!("_rate <= 0, no count to stream to port");
            }
            for _x in 0..rate {
                let mut record = data_record_builder
                    .build_record()
                    .get_serde_formatted_json();
                record.push_str("\n");
                port_streamer.connect_and_send(record);
            }
            if !port_rate_fixed {
                rate = Self::get_rand_rate(messages_per_second);
            }
        }
    }

    fn get_rand_rate(messages_per_second: usize) -> usize {
        let mut _min_rate = messages_per_second - (messages_per_second as f64 * 0.2) as usize;
        let mut _max_rate = messages_per_second + (messages_per_second as f64 * 0.2) as usize;
        dbg!(_min_rate.clone());
        dbg!(_max_rate.clone());
        if _min_rate <= 1 {
            _min_rate = 1;
        }
        if _max_rate <= _min_rate {
            let results = _max_rate / 2;
            dbg!(results.clone());
            return results;
        } else {
            let results = rand::thread_rng().gen_range(_min_rate, _max_rate) / 2;
            dbg!(results.clone());
            return results;
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_get_rand_rate() {
        let value = 1;
        loop {
            dbg!(PortDataFactory::get_rand_rate(value));
        }
    }
}
