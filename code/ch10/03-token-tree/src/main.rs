
macro_rules!  write_html {
    ($w: expr, ) => (());

    ($w: expr, $e: tt) => (write!($w, "{}", $e).expect("Não foi possível escrever o HTML"));

    ($w: expr, $tag: ident [  $($inner: tt)* ] $($rest: tt)* ) => {
        write!($w, "<{}>", stringify!($tag)).expect("Não foi possível escrever o HTML");
        write_html!($w, $($inner)*);
        write!($w,"</{}>", stringify!($tag)).expect("Não foi possível escrever o HTML");
        write_html!($w, $($rest)*);
    };
}

fn main() {
    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out, 
        html[
            head[title["Livro de Rust"]]
                body[
                    h1["Autores"]
                        p["Marcelo Castellani"]
                        p["William 'PotHix' Molinari"]
                ]
        ]
    );

    println!("{}", out);
}
