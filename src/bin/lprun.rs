//! simple basic cli interface for lprun library

extern crate clap;
extern crate lprun;
extern crate ansi_term;

use lprun::interface;

fn main() {  
  // builds the app
  let app = interface::app()
    .get_matches();

  // processess the arguement matches.
  match interface::process(&app) {
    Err(error) => { println!("{}",error); }
    Ok(_) => { }
  }
}