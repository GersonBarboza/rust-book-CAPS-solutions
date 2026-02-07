fn main() {
    let a = String::from("Rust");

    for chr in a.chars() {
        println!("char: {}", chr)
    }

    for bte in a.bytes() {
        println!("byte: {}", bte)
    }

    for (index, chr) in a.chars().enumerate() {
        println!("{} -> {}", index, chr)
    }

}
