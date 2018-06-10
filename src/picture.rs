use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn draw(pic: Vec<String>) -> () {
    println!("{}", pic.into_iter().fold(String::new(), |acc, ln| acc + &ln))
}

pub fn generate_picture((height, len): (usize, usize)) -> Vec<String> {
    let mut pic = generate_art(height,len);
    let topbot_str: String = iter::repeat("#").take(len+6).collect();
    pic.reverse();    //push to top
    pic.push(format!("{}\n",topbot_str));
    pic.reverse();    //push to bottom
    pic.push(format!("{}\n",topbot_str));
    pic
}

fn frame_wrap_ln(ln: String) -> String {
    format!("## {} ##\n", ln)
}

fn generate_art(height: usize, len: usize) -> Vec<String> {
    let pic: Vec<String> = iter::repeat(())
        .map(|()| random_ascii_line(len))
        .take(height)
        .collect();
    pic
}

fn random_ascii_line(len: usize) -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect();
    frame_wrap_ln(chars)
}
