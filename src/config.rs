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
    // text: Option<Vec<String>>, // use Vec<String> instead of vec to accept one or more values 
    #[arg(required = true)]
    text: Vec<String>,
}

pub fn print_join(c: &Config) {

    
    let ending = if c.omitnewline {""} else {"\n"}; // `if` is an expression in rust 

    // match &c.text {
    //     Some(text) => print!("{}{}", text.join(" "), ending),
    //     None => print!("{}", ending)
    // }
    print!("{}{}", c.text.join(" "), ending)
}
    