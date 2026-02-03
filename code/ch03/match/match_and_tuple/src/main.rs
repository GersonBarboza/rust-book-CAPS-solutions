
/*
#[allow(unused_variables)]

// esta versão precida da diretiva #[allow(unused_variables)] 
// para evitar warning devido ao uso da variável x
fn check_tuple(t: (i32, i32)) -> () {
    match t {
        (0, 0) => println!("Os dois são zero."),
        (0, x) => println!("O primeiro é zero."),
        (x,0) => println!("O segundo é zero."),
        _ => println!("Nenhum zero.")
    }
}
*/


// colocar "_" no iníco do nome de uma variável avisa ao compilador
// que a variável não será usada. Portanto troca 'x' por '_x' torna
// desncessário o uso da diretiava #[allow(unused_variables)]
// Observação: se a variável for nomeada deste jeito e for usada, ok.
// Vide exemplo de '_y' no final da função main.
fn check_tuple(t: (i32, i32)) -> () {
    match t {
        (0, 0) => println!("Os dois são zero."),
        (0, _x) => println!("O primeiro é zero."),
        (_x,0) => println!("O segundo é zero."),
        _ => println!("Nenhum zero.")
    }
}


fn main() {
    check_tuple((0,10));
    check_tuple((33,0));
    check_tuple((0,0));
    check_tuple((8,12));
    
    let _y = "Sou uma variável cujo nome começa com '_'";
    println!("{}", _y)
}
