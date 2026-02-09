use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Info {
    name: &'static str,
    age: i32,
    rating: i32,
}

fn main() {
    test01();
    test02();
}


fn test01() {
    println!("\n-------------- TEST 01 ----------------");

    let friend01 = Info {
        name: "Rachel Karen Green",
        age: 30,
        rating: 10,
    };
    match write_info_with_err(&friend01) {
        Err(e) => println!("Ops, algo deu errado -> {}", e),
        Ok(()) => println!("Tudo em seu devido lugar"),
    };
}

fn write_info_with_err(info: &Info) -> io::Result<()> {
    let mut file = match File::create("best_friends.txt") {
        Err(e) => return Err(e),
        Ok(f) => f,
    };
    if let Err(e) = file.write_all(format!("nome: {}\n", info.name).as_bytes()) {
        return Err(e);
    }
    if let Err(e) = file.write_all(format!("idade: {}\n", info.age).as_bytes()) {
        return Err(e)
    }
    if let Err(e) = file.write_all(format!("avaliação: {}\n", info.rating).as_bytes()) {
        return Err(e)
    }
    Ok(())
}

fn test02() {
    println!("\n-------------- TEST 02 ----------------");

    let friend01 = Info {
        name: "Rachel Karen Green",
        age: 30,
        rating: 10,
    };
    match write_info(&friend01) {
        Err(e) => println!("Ops, algo deu errado -> {}", e),
        Ok(()) => println!("Tudo em seu devido lugar"),
    };
}

fn write_info(info: &Info) -> io::Result<()> {
    let mut file = File::create("best_friends.txt") ?;

    file.write_all(format!("nome: {}\n", info.name).as_bytes()) ?;
    file.write_all(format!("idade: {}\n", info.age).as_bytes()) ?;
    file.write_all(format!("avaliação: {}\n", info.rating).as_bytes()) ?;
    Ok(())
}
