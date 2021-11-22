use std::{ env };

use minigrep;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        minigrep::handle_error(err);
    });

    println!("Searching for {}", config.query);
    
    println!("In file {}", config.filename);
    
    if let Err(e) = minigrep::run(config) {
        minigrep::handle_error(&e.to_string());
    }
}
