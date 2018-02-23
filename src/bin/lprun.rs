//! simple basic cli interface for lprun library

extern crate clap;
extern crate lprun;
extern crate ansi_term;

use lprun::interface;
use std::env;

fn main() {  
  // builds the app
  let app = interface::app()
    .arg(clap::Arg::with_name("debug").long("debug").help("Shows additional information about commands run."))
    .get_matches();

  if app.is_present("debug") { env::set_var("OUTPUT_DEBUG_ENABLED","true"); }

  // processess the arguement matches.
  match interface::process(&app) {
    Err(error) => { println!("{}",error); }
    Ok(_) => { }
  }
}