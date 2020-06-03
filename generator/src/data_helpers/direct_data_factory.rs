use crate::args_helpers::argument_helper::ArgumentHelper;
use crate::data_helpers::json_test_data::helpers::json_test_data_factory::JsonTestDataFactory;
use crate::data_helpers::live_data::helpers::live_data_factory::LiveDataFactory;

use log::{debug};

pub struct DirectDataFactory {}

impl DirectDataFactory {
    /// # About
    /// Starting function for directing path for JsonTestData
    /// or LiveData
    ///
    /// # Status
    /// Needs to be refactored to leverage generics
    /// 
    pub fn start(argument_helper: ArgumentHelper) {
        debug!("start");
        // validate json data structures or live data structure
        if argument_helper.validate_json {
            JsonTestDataFactory::new(argument_helper).start();
        } else {
            LiveDataFactory::new(argument_helper).start();
        }
    }
}
