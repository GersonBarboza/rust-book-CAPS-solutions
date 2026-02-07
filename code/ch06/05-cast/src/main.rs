fn main() {
    cast_test("42");
    cast_test("ERRO")
}


//-gb- O cast é feito usando o método 'parse' o qual retorna um 'Result'
fn cast_test(word: &'static str) {
    let a = String::from(word);

    let b = match a.parse::<i8>() {
        Ok(c) => c + 1,
        Err(_d) => 0,
    };

    println!("{} + 1 = {}", a, b)
}
