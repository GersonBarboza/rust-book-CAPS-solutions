extern crate futures;


fn main() {
    let future = example_task();

    for i in 0..10 {
        println!("Nada acontece por aqui => {}", i)
    }

    futures::executor::block_on(future);
}

async fn async_fibonacci(n: u32) -> u32 {
    if n == 0 {
        panic!("Zero não é argumento válido!");
    } 
    else 
    if n == 1 {
        return 1;
    }

    let mut sum: u32 = 0;
    let mut last: u32 = 0;
    let mut curr: u32 = 1;

    for _ in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    sum
}

async fn example_task() {
    let  num1 = 10;
    let fibo = async_fibonacci(num1).await;
    println!("fivonacci({}) => {}", num1, fibo);
}
