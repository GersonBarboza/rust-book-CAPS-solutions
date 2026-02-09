use std::thread;

fn main() {
    //test01();
    test02();
}

/*
fn test01() {
    println!("\n----------- TEST 01 ---------------");

    let _a = thread::spawn(|| {
        println!("Olá da thread 'A'!")
    });

    let _b = thread::spawn(|| {
        println!("Olá da thread 'B'!")
    });

    //-gb- Há uma chance de não imprimir nenhum mensagem caso a 
    // thread principal termina antes das threads '_a' e '_b'.
}
*/

fn test02() {
    println!("\n----------- TEST 02 ---------------");

    let _a = thread::spawn(|| {
        println!("Olá da thread 'A'!")
    });

    let _b = thread::spawn(|| {
        println!("Olá da thread 'B'!")
    });

    //-gb- Para resolver o problema mencionado no 'test02()'
    // usa-se o método 'join'.

    let _ = _a.join();
    let _ = _b.join();
}


