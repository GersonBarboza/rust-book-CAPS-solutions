fn main() {
    check_number(0);
    check_number(3);
    check_number(8);
}


fn check_number(num :i16) -> () {
    match num {
        0 => println!("Zero"),
        1 | 3 | 5 | 7 | 9 | 11 => println!("Primo"),
        _ => println!("Outro número qualquer"),
    }
}