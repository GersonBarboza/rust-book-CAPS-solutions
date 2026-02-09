macro_rules! distance {
    ($a: ident, $b: expr, $c: expr) => {
        let $a = {
            if $b >= $c {
                $b - $c
            } else {
                $c - $b
            }
        };
    }
}

fn _test_01() {
    println!("-------------- TEST 01 ------------");

    distance!(x, 5, 3);
    distance!(y, 3, 5);
    println!("{}, {}", x, y);

    distance!(x, 10+3, 112+33);
    distance!(y, 3-50, 1024+5);
    println!("{}, {}", x, y);

}

macro_rules! distance2 {
    ($a: ident,
    v1 => $b: expr,
    v2 => $c: expr ) => {
        let $a = {
            if $b >= $c {
                $b - $c
            } else {
                $c - $b
            }
        };
    }
}


fn _test_02() {
    println!("-------------- TEST 02 ------------");

    //-gb- Não entendi a vantatgem de usar esse 'v1' e 'v2' no 'distance2!' :o(
    distance2!(x, v1=> 5+4, v2 => 11-3);
    distance2!(y, v1 => 3, v2 => 5);
    println!("{}, {}", x, y);

}


fn main() {
    _test_01();
    _test_02();
}