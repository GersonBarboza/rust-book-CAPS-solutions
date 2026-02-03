

enum BookFormat {
    PaperBack,
    Ebook
}

struct Book {
    isbn: i32,

    //#[allow(dead_code)]
    title: &'static str,

    //#[allow(dead_code)]
    format: BookFormat
}

impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.isbn == other.isbn
    }
}

fn main() {

    let b1 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis - A Sociedade do Anel",
        format: BookFormat::PaperBack
    };

    let b2 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis - As Duas Torees",
        format: BookFormat::Ebook
    };

    let b3 = Book {
        isbn: 1876543210,
        title: "O Hobit - Uma Viagem Inesperada",
        format: BookFormat::Ebook
    };


    println!("{}", b1 == b2);
    println!("{}", b2 == b3);
    println!("{}", b1 == b3);

}
