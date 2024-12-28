use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "miniecho")]
#[command(version = "0.1")]
#[command(about = "A lesser echo utility written in rust", long_about = None)]
pub struct Config {

    /// If set true, omit Newline character at the end
    #[arg(
        short = 'n',
        long = "omitnewline",
        required = false,
        default_value_t = false
    )]
    omitnewline: bool,

    /// Actual inputs for miniecho to display
    #[arg(
        required = true
    )]
    text: Vec<String>, // use Vec<String> instead of vec to accept one or more values 
}

pub fn print_join(c: &Config) {
    let mut ending = "\n";

    if c.omitnewline {
        ending = "";
    }
    println!("{}{}", &c.text.join(" "), ending);
}