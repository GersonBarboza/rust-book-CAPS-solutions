fn main() {
    concat_with_operator();
    concat_with_push_str();
    concat_char_using_push();
}



fn concat_with_operator() {


    println!("\n==>> Using operator '+' and '+=':");

    let publisher = "Casa do Código".to_string();
    let authors = String::from("Marcelo Catellani e Willian Molinari");
    let book: String = "\"A Linguagem Rust\"".into();

    let mut sentence = String::from("Este é o llivro ");
    sentence += &book;
    sentence += " da ";
    sentence += &publisher;
    sentence += ", \nescrito por ";
    sentence += &authors;

    println!("{}", sentence);
}

fn concat_with_push_str() {
    println!("\n==>> Using method 'push_str':");

    let publisher = "Casa do Código".to_string();
    let authors = String::from("Marcelo Catellani e Willian Molinari");
    let book: String = "\"A Linguagem Rust\"".into();

    let mut sentence = String::from("Este é o llivro ");
    sentence.push_str(&book);
    sentence.push_str(" da ");
    sentence.push_str(&publisher);
    sentence.push_str(",\nescrito por ");
    sentence.push_str(&authors);

    println!("{}", sentence);
}


fn concat_char_using_push() {
    println!("\n==>> Concatenating chars using method 'push':");

    let mut sentence = String::from("Olá, ");
    sentence.push('R');
    sentence.push('u');
    sentence.push('s');
    sentence.push('t');
    sentence.push('!');

    println!("{}", sentence);

}

