use std::thread;

fn main() {
    //_test02();
    _test03();
}

/*
//-gb- Este código não compila porque não posso passar um valor pra 
// thread sem usar explicitamente o método 'move'.
fn test01() {
    println!("\n------------- TEST 01 -------------");

    let value = 10;

    let a = thread::spawn(|| {
        println!("{}", value)
    });

    let _ = a.join();
}
*/

fn _test02() {
    println!("\n------------- TEST 02 -------------");

    let value = 10;

    let a = thread::spawn( move || {
        println!("value = {}", value)
    });

    let _ = a.join();
}

fn _test03() {
    println!("\n------------- TEST 03 -------------");

    let mut value = 10;

    let a = thread::spawn( move || {
        value += 123;
        println!("[thread A] value = {}", value)
    });

    let b = thread::spawn( move || {
        value += 1;
        println!("[thread B] value = {}", value)
    });

    let _ = a.join();
    let _ = b.join();

    println!("[main thread] value = {}", value)

}

