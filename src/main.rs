use std::io;
fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text)
    .expect("Failed to read line");
    let words: Vec<&str> = input_text.split_whitespace().collect();
    let nambers: Vec<i32> = words.iter().map(|&w| w.to_string().parse().unwrap()).collect();
    println!("{}", nambers.iter().sum::<i32>());
}
