use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> () {
    tokio::spawn(async {example_task().await});

    loop {
        sleep(Duration::from_millis(500)).await;
        println!("Nada acontece por aqui");
    }

}

async fn async_fibonacci(n: u64) -> u64 {
    if n == 0 {
        panic!("Zero não é argumento válido!");
    } 
    else 
    if n == 1 {
        return 1;
    }

    let mut sum: u64 = 0;
    let mut last: u64 = 0;
    let mut curr: u64 = 1;

    for _ in 1..n {
        sleep(Duration::from_millis(100)).await;
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    sum
}

async fn example_task() {
    let  num1 = 75;
    let fibo = async_fibonacci(num1).await;
    println!("fibonacci({}) => {}", num1, fibo);
}
