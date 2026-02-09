macro_rules! sum {
    ($a: ident, $($x: expr), *) => {
        let $a = {
            let mut temp = 0;
            $(temp = temp + $x;)*  //-gb- executa para cada parâmetro o código dentro de $()*
            temp
        };
    }
}

fn test01() {
    println!("\n-------------- TEST 01 ---------------");

    sum!(v, 1, 2);
    sum!(x, 1, 2, 3, 4, 5, 6, 7);
    sum!(y, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 );
    println!("v={},  x={},  y={}", v, x, y);

    //-gb- Não funiona quando os outros números não i32, 
    //     devido à inicialização 'let mut temp = 0' 
    //     que por default faz 'temp: i32'.
    //sum!(z, 1, 2.0, 3.5, 0.5);
    //println!("z= {}", z);

}


macro_rules! sum2 {
    ($a: ident, 
     $b: expr,
     $($x: expr), *) => {
        let $a = {
            let mut temp = $b;
            $(temp = temp + $x;)*  //-gb- executa para cada parâmetro o código dentro de $()*
            temp
        };
    }
}

fn test02() {
    println!("\n-------------- TEST 02 ---------------");

    sum2!(v, 1.0, 2.5);
    sum2!(x, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5);
    sum2!(y, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 );
    println!("v={},  x={},  y={}", v, x, y);

}


fn main() {
    test01();
    test02();
}