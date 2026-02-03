

fn main () {
    let name : &'static str = "Adalbertolino";
    
    example_vowel_consonant_count(name);
    
    example_with_atbinding(name);


}


//
// Exemplo sem uso do at-binding
fn example_vowel_consonant_count(name :&'static str) -> () {
    let mut vowel_count = 0;
    let mut cosonant_count = 0;

    for a in name.chars() {
        match is_vowel_or_consonant(a) {
            'v' => vowel_count += 1,
            'c' => cosonant_count += 1,
            _ => ()
        }
    }

    println!("O nome {} tem {} vogais e {} consoantes.",
                name, vowel_count, cosonant_count);

}


fn example_with_atbinding(name :&'static str) -> () {
    for a in name.chars() {
        match is_vowel_or_consonant(a) {
            r @ 'v' => println!("'{}' is {}", a, r),
            r @ 'c' => println!("'{}' is {}", a, r),
            _ => ()
        }
    }

}


fn is_vowel_or_consonant(c: char) -> char {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => 'v',
        'A' | 'E' | 'I' | 'O' | 'U' => 'v',
        _ => 'c'
    }
}
