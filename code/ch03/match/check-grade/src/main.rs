fn main() {
    check_grade(0.0);
    check_grade(3.2);
    check_grade(5.1);
    check_grade(8.3);
}


fn check_grade(grade :f32) -> () {
    match grade {
        0.0..=4.8   => println!("Não aprovado"),
        4.9..=5.9   => println!("De exame"),
        6.0..=10.0  => println!("APROVADO"),
        _           => println!("Nota inválida")
    }
}

