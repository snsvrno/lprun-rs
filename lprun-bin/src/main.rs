//! simple basic cli interface for lprun library
extern crate clap;
extern crate lprun;

use lprun::interface;
use std::env;

fn main() {
    // builds the app
    let app = interface::app()
        .arg(clap::Arg::with_name("debug").long("debug").help("Shows additional information about commands run."))
        .get_matches();

    // starts the loggers & sets the filter level for the logs
    match pretty_env_logger::formatted_builder() {
        Err(error) => { println!("Failed to start logging: {}",error); },
        Ok(mut builder) => {
            let level = if app.is_present("debug") { 
                log::LevelFilter::Info 
            } else { 
                log::LevelFilter::Error 
            };

            builder
                .filter(None,level)
                .init();
        }
    }

    // processess the arguement matches.
    match interface::process(&app) {
        Err(error) => { println!("{}",error); }
        Ok(_) => { }
    }
}