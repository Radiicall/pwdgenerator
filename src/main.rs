use rand::prelude::*;

fn main() {
    let list = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", 
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
    "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
    "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1",
    "2", "3", "4", "5", "6", "7", "8", "9", "!", "@", "#", "$", "%", "^", "&",
    "*", "(", ")", "-", "_", "+", "=", ",", ".", "?", ";", ":", "'",];
    let mut rng = thread_rng();
    let mut length = String::new();
    println!("Enter length: ");
    std::io::stdin().read_line(&mut length).unwrap();

    let length: usize = length.trim().parse().unwrap();

    for _ in 0..length {
        let index: usize = rng.gen_range(0..82);
        let letter = list[index];
        print!("{}", letter);
    }
    println!();

}
