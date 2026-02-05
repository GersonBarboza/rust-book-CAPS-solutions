#[derive(Debug)]
struct Contact {
    name: &'static str,
    phone: &'static str
}

fn main() {
    //test_1();
    //test_2();
    //test_3();
    //test_4();
    //test_5();
    //test_6();
    //test_7();
    test_8();
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
    println!("{:?}", agenda);
}

fn test_2() {
    println!("\nTEST 2");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];
    println!("Agenda com dois contatos:\n{:?}", agenda);

    let contact_3 = Contact {
        name: "Nobody",
        phone: "+55 (11) 9-9999-0000"
    };

    agenda.push(contact_3);
    println!("Agenda com TRÊS contatos:\n{:?}", agenda);
}

fn test_3() {
    println!("\nTEST 3");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];
    println!("Agenda com dois contatos:\n{:?}", agenda);

    //Como pop retorna o elemento deletado, para ignorar esse retorno uso '_'
    let _ = agenda.pop();
    println!("Agenda com um contato:\n{:?}", agenda);
}

fn test_4() {
    println!("\nTEST 4");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];
    println!("Contato 1: {:?}", agenda[0]);
    println!("Contato 2: {:?}", agenda[1]);
}

fn test_5() {
    println!("\nTEST 5");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];

    let contact_3 = Contact {
        name: "Nobody",
        phone: "+55 (11) 9-9999-0000"
    };

    agenda.push(contact_3);

    for contact in agenda {
        println!("{:?}", contact);
    }
}

fn test_6() {
    println!("\nTEST 6");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];

    let contact_3 = Contact {
        name: "Nobody",
        phone: "+55 (11) 9-9999-0000"
    };

    agenda.push(contact_3);


    let mut agenda_iter = agenda.iter();

    println!("{:?}", agenda_iter.next().unwrap());
    println!("{:?}", agenda_iter.next().unwrap());
    
}

fn test_7() {
    println!("\nTEST 7");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];

    let contact_3 = Contact {
        name: "Nobody",
        phone: "+55 (11) 9-9999-0000"
    };

    agenda.push(contact_3);


    let mut agenda_iter = agenda.iter();

    loop {
        let item = agenda_iter.next();

        match item {
            None => break,
            Some(contact) => println!("{:?}", contact)
        };

    }
    
}

fn test_8() {
    println!("\nTEST 8");

    let contact_1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let contact_2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };

    let mut agenda = vec![contact_1, contact_2];

    let contact_3 = Contact {
        name: "Nobody",
        phone: "+55 (11) 9-9999-0000"
    };

    agenda.push(contact_3);


    let mut agenda_iter = agenda.iter();

    loop {
        let item = agenda_iter.next();

        if let Some(contact) = item {
            println!("{:?}", contact);
        } else {
            break;
        }

    }
    
    
}


