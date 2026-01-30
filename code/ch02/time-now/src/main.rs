use chrono::Local;
fn main() {
    let t = Local::now();
    println!("agora: {}", t);

    let x = true;
    if x == true {
        println!("x é true.")
    }
}
