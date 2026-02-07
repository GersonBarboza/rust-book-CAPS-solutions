fn main() {
    test01();
    test02();
    test03();
}


fn test01() {
    println!("\n==> TEST 01 --------------->");

    let name = String::from("Luke Skywalker");
    let space = name.find(" ").unwrap_or(0);

    println!("Espaço na posição: {}", space)
}

fn test02() {
    println!("\n==> TEST 02 --------------->");

    let mut name = String::from("Luke Skywalker");
    let space = name.find(" ").unwrap_or(0);

    let first_name: String = name.drain(..space).collect();

    println!("Primeiro nome: \"{}\"", first_name);
    println!("Restante do nome: \"{}\"", name)
}

fn test03() {
    println!("\n==> TEST 03 --------------->");

    let mut name = String::from("Luke Skywalker");
    let space = name.find(" ").unwrap_or(0);

    let first_name: String = name.drain(..space).collect();
    let space = name.find(" ").unwrap_or(0); //-gb- usando shadowing

    let last_name: String = name.drain((space+1)..).collect();

    println!("Primeiro nome: \"{}\"", first_name);
    println!("Último nome  : \"{}\"", last_name);

    println!("A string original está vazia? {}", name.is_empty())

}
