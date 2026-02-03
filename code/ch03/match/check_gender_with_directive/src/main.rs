#[allow(dead_code)]
enum Gender {
    Male,
    Female,
    Other,
}


use Gender::Female as gfemale;

fn main() {
    let gender = Gender::Male;
    match gender {
        Gender::Other => println!("Outro"),
        Gender::Male => println!("Male"),
        gfemale => println!("Female")
    }
}
