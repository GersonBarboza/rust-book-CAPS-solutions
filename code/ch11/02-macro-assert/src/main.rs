fn main() {
    //_test01();
    _test02();
}

fn _test01() {
    println!("\n----------- TEST 01 -----------");
    
    assert!(2 + 2 == 8 / 2);
    assert!(true);
    assert!('a'.is_alphabetic());
    assert!('1'.is_numeric());
    assert!('a' == 'b');
}
fn _test02() {
    println!("\n----------- TEST 02 -----------");

    assert_eq!(2 + 2, 8 / 2);
    assert_eq!(true, 'a' == 'a');
    assert_eq!(true, 'a' == 'b');
}
