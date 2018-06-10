extern crate clap;
extern crate rand;

mod picture;

use clap::{Arg, App};

fn main() {
    //CLI
    let matches = App::new("asciibyaccident")
        .version("0.1.0")
        .about("Generate ascii art on random")
        .arg(
            Arg::with_name("aspectratio")
                .short("r")
                .long("ratio")
                .value_name("ratio")
                .help(
                    "sets the bounds for the generated image heightxlength in chars",
                )
                .takes_value(true),
        )
        .get_matches();

    let ratio_in = matches.value_of("aspectratio").unwrap_or("58x200");
    let ratio_t = match ratio_in {
        "58x200" => (58, 200),
        s => parse_ratio(s.to_ascii_lowercase()),
    };

    let fl = picture::generate_picture(ratio_t);
    picture::draw(fl)
}

fn parse_ratio(s: String) -> (usize, usize) {
    let mut s_vector: Vec<&str> = s.split('x').collect();
    // assign entered str values
    let s_len = s_vector.pop().unwrap();
    let s_height = s_vector.pop().unwrap();
    // parse to int
    let height: usize = s_height.parse().unwrap();
    let len: usize = s_len.parse().unwrap();
    let hl = (height, len);
    hl
}
