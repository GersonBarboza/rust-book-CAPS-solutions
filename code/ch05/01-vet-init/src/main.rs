fn main() {
    init_01();
    init_02();
    init_03();
}

fn init_01() {
    let vector = vec![1, 2, 3];
    println!("{:?}", vector);
}

fn init_02() {
    let vector: Vec<i32> = (1..4).collect();
    println!("{:?}", vector);
}

fn init_03() {
    let vector = vec![0;5];
    println!("{:?}", vector);
}


