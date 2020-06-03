extern crate rand;

use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::json_test_data::helpers::data_object_test_record_builder::DataObjectTestRecordBuilder;
use crate::synth_library::data_exporters::port_streamer::PortStreamer;

use rand::Rng;

pub struct TestPortDataFactory {}

impl TestPortDataFactory {
    pub fn stream_port_data(
        data_record_builder: DataObjectTestRecordBuilder,
        arguments: &ArgumentHelper,
    ) {
        let mut _rate = 0;
        let half_a_second = std::time::Duration::new(0, 500000000);
        let _messages_per_second = arguments.count;
        let _port_streamer = PortStreamer::new(
            &arguments.src_ip_address,
            &arguments.src_port,
            &arguments.dst_port,
        );

        loop {
            std::thread::sleep(half_a_second);
            match arguments.port_rate.as_ref() {
                ArgumentHelper::PORT_RATE_FIXED => {
                    _rate = _messages_per_second / 2;
                }
                ArgumentHelper::PORT_RATE_VARIABLE => {
                    let mut _min_rate =
                        _messages_per_second - (_messages_per_second as f64 * 0.2) as usize;
                    let mut _max_rate =
                        _messages_per_second + (_messages_per_second as f64 * 0.2) as usize;
                    if _min_rate < 1 {
                        _min_rate = 1;
                    }
                    if _max_rate < _min_rate {
                        _max_rate = _min_rate;
                    }
                    _rate = rand::thread_rng().gen_range(_min_rate, _max_rate) / 2;
                }
                &_ => {
                    panic!("_port_rate found &_");
                }
            }
            println!(
                "{} records generated and streamed to {}",
                _rate.clone(),
                _port_streamer.get_dst()
            );
            for _x in 0.._rate {
                // let record = data_record_builder.build_record().get_formatted_record();
                let mut record = data_record_builder
                    .build_record()
                    .get_serde_formatted_json();
                record.push_str("\n");
                _port_streamer.connect_and_send(record);
            }
        }
    }
}
