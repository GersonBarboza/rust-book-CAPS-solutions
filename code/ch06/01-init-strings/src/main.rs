fn main() {

    init_from_constant();
    init_form_byte_vector();
}

fn init_from_constant() {

    let editora = "Casa do Código".to_string();
    let autor = String::from("Marcelo Castallani e William Molina");
    let livro: String = "A Linguagem Rust".into();

    println!("Este é o livro '{}' da {}, \nescrito por {}", 
            livro, editora, autor);

}

fn init_form_byte_vector() {
    let vec = vec![82, 117, 115, 116];
    
    let a = String::from_utf8(vec).unwrap();
    //Aqui foi usado o método 'unwrap()' porque 'from_utf8()' retorna um 'Result'.

    println!("{}", a);
}

