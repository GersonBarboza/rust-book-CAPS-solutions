fn main() {
    test01();
    test02();
    test03();
    test04();
    test05();
    test06();
}

fn test01() {
    println!("\n==>> TEST 01 --------------->");

    let a = String::with_capacity(255);
    let b = String::from("ABC");

    println!("a: {} -> {}, len = {}", a, a.capacity(), a.len());
    println!("b: {} -> {}, len = {}", b, b.capacity(), b.len());
}

fn test02() {
    println!("\n==>> TEST 02 --------------->");

    let mut a = String::from("Marcelo");
    a.reserve(20);
    println!("a: {} -> {}, len = {}", a, a.capacity(), a.len());
}

fn test03() {
    println!("\n==>> TEST 03 --------------->");

    let mut a = String::from("Rust");
    a.reserve(10);
    println!("SEM 'shrink' => a: {}, capacity={}, len={}", a, a.capacity(), a.len());

    a.shrink_to_fit();
    println!("APÓS 'shrink' => a: {}, capacity={}, len={}", a, a.capacity(), a.len());
}

fn test04() {
    println!("\n==>> TEST 04 --------------->");

    let mut a = String::from("Rust");
    a.reserve(50);

    a.push_str(" rules");
    println!("-> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());

    a.shrink_to_fit();
    println!("'a.shirink_to_fit()'-> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());

    a.truncate(4);
    println!("'a.truncate(4)'-> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());

    a.shrink_to_fit();
    println!("'a.shirink_to_fit()'-> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());

    a.clear();
    println!("'a.clear()'-> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());
}

fn test05() {
    println!("\n==>> TEST 05 --------------->");

    let mut a = String::from("Rust");
    a.reserve(50);

    println!("ANTES DA REMOÇÃO -> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());

    a.remove(2);
    println!("'a.remove(2)'-> a: \"{}\" -> capacity={}, len={}", a, a.capacity(), a.len());
}

fn test06() {
    println!("\n==>> TEST 06 --------------->");

    let mut a = String::from("Rust");

    for _x in 0..a.len() {
        let ret = a.pop();
        match ret {
            Some(char) => println!("Pop -> {}", char),
            None => println!("Sem mais caracteres..."),
            //-gb- Observar que o println! do None não executa porque o for só roda 4 vezes.
        }
    }
}

