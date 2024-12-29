use clap::Parser;

mod config;

fn main() {
    let args = config::Config::parse();

    // println!("Args: {:?}", &args);
    config::print_join(&args);
}
