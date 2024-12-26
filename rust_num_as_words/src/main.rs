use std::env::args;

use num_to_str::num_as_words;
mod num_to_str;

fn main() {
    let num = args()
        .nth(1)
        .unwrap_or("0".to_string())
        .trim()
        .parse::<i128>()
        .expect("Please enter a correct number");

    println!("{:#?}", num_as_words(num));
}
