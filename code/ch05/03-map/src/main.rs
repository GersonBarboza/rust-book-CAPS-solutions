#[derive(Debug)]
struct Contact {
    name: &'static str,
    phone: &'static str
}

fn main() {
    test_1();

}


fn test_1() {
    println!("\nTEST 1");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let agenda = vec![contact_1, contact_2];

    //-gb- Deste modo também funciona, pois assim o collect() sabe que tipo retornar.
    // let names:Vec<&'static str> = agenda.iter()
    //         .map(|contact| {contact.name})
    //         .collect();

    let names = agenda.iter()
            .map(|contact| {contact.name})
            .collect::<Vec<_>>();

    println!("Nomes: {:?}", names);


    let numbers = agenda.iter()
            .map(|contact| {contact.phone})
            .collect::<Vec<_>>();

    println!("Números: {:?}", numbers);


}

