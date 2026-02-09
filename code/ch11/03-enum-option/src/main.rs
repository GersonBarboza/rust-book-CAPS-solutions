
fn main(){
    test01();
    test02();
}




struct Fibonacci {
    curr: i32,
    next: i32,
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        //Esta sequencia de Fibonacci irá somente até 21
        if self.curr > 21 {
            None
        } else {
            Some(self.curr)
        }
    }
}

fn test01() {
    println!("\n--------------- TEST 01 -------------");

    let fib1 = Fibonacci{curr: 1, next: 1};
    for i in fib1.take(4) {
        println!("> {}", i);
    }


    //-gb- Observar que no código abaixo o loop só executa 3 vezes,
    // apesar do 'take(4)' pedir para fazer 4 vezes. É que neste caso,
    // quando 'Option.None' é retonado, a iteração acaba e tudo ok.
    // Mas se houvesse um uso para o retorno da função, o 'Option.None'
    // causaria uma exceção, como será mostrado no 'test02()'.
    let fib2 = Fibonacci{curr: 1, next: 1};
    for i in fib2.skip(4).take(4) {
        println!("> {}", i)
    }
}

// Demostrando o uso de 'Option' onde retornar 
// 'Option.None' causará uma exceção.
fn test02() {
    println!("\n--------------- TEST 02 -------------");

    // Chamar 'unwrap()' para 'even_test(.)' com um 'number' sendo par 
    // é OK, pois ele retorna 'Some(number)'
    println!("Esse número é par: {}", even_test(22).unwrap());

    println!("A próxima instrução gerará um panic:");

    // Chamar 'unwrap()' para 'even_test(.)' com um 'number' sendo ÍMPAR 
    // gera uma exceção (panic), pois neste caso o retorno é 'Option.None'.
    println!("Esse número é par: {}", even_test(23).unwrap());

}
fn even_test(number: i32) -> Option<i32> {
    if number % 2 == 0 {
        Some(number)
    } else {
        None
    }
}