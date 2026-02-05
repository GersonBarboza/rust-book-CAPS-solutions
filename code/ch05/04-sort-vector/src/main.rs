use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy,Debug)]
struct Contact {
    name: &'static str,
    phone: &'static str
}

// impl Ord for Contact {
//     fn cmp(&self, other: &Contact) -> Ordering {
//         (self.name).cmp(&(other.name))
//     }
// }

impl Ord for Contact {
    fn cmp(&self, other: &Contact) -> Ordering {
        self.name.cmp(&(other.name))
    }
}

fn main() {
    sort_test();
    first_last_test();

}

fn sort_test() {
    println!("\nSORT TEST ---------------------------------------------\n");

    let c1 = Contact {
        name: "Marcelo",
        phone: "+55 (11) 9-1234-5678"
    };

    let c2 = Contact {
        name: "Willian",
        phone: "+55 (11) 9-8765-4321"
    };


    let c3 = Contact {
        name: "Joey",
        phone: "+55 (11) 9-9999-0000"
    };

    let c4 = Contact {
        name: "Lidiane",
        phone: "+55 (11) 9-9999-1234"
    };

    let c5 = Contact {
        name: "Alda",
        phone: "+55 (11) 9-9086-4208"
    };

    let mut agenda = vec![c1, c2, c3, c4, c5];

    println!("Agenda antes de ordernar:");
    println!("{:?}", agenda);

    agenda.sort();

    println!("\nAgenda APÓS ordernar:");
    println!("{:?}", agenda);


}

fn first_last_test() {
        println!("\nFIRST LAST TEST -----------------------------------\n");

    let numbers = vec![134, 12, 3, 2, 6, 42];
    println!("first vector element: {:?}", numbers.first());
    println!("last vector element: {:?}",  numbers.last());

}