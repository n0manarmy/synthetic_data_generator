extern crate synth_library;

mod args_helpers;
mod data_exporters;
mod data_helpers;
mod utils;

use log::{debug, info};

use args_helpers::{
    argument_helper::ArgumentHelper,
    help::Help
};
use data_helpers::direct_data_factory::DirectDataFactory;
use utils::log_builder::LogBuilder;
use std::{env, process::exit};

/// Collect command line arguments together and then validate count.
/// Print help file or continue building the StringValues and
/// DataRecordBuilder helpers.
/// Pick path for exporting generated data, either file or port stream.
fn main() {
    // Collects arguments from system at runtime and parses to vector
    debug!("Reading command line args");
    let args: Vec<String> = env::args().collect();

    // validate that values were provided, otherwise print help and exit
    if args.len() <= 1 {
        debug!("No command line args, supplied, printing help");
        Help::print_help();
        exit(1);
    }

    // parse arguments from runtime into argument helper object
    let argument_helper: ArgumentHelper = ArgumentHelper::new().parse_args(args);
    if argument_helper.help {
        Help::print_help();
        exit(1);
    }
    LogBuilder::build_logger(&argument_helper.log_level);
    info!("Logger started");


    // DataFactory::start(DataDestination::<Type>, DataModel::<Type>)
    DirectDataFactory::start(argument_helper);

    info!("Finished!");
}
