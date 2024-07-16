use std::io;
fn main() {
    println!("Enter the text to encrypt:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");
    println!("Enter the shift number:");
    let mut shift_str = String::new();
    io::stdin().read_line(&mut shift_str).expect("Failed to read line");
    let shift: u8 = match shift_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid shift number, using default shift 3");
            3
        }
    };
    println!("Encrypted text: {}", encrypt_caesar(&text, shift));
}
fn encrypt_caesar(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (ch as u8).wrapping_sub(base);
            let shifted = (offset + shift) % 26;
            let encrypted_char = (base + shifted) as char;
            result.push(encrypted_char);
        } else {
            result.push(ch);
        }
    }
    result
}
