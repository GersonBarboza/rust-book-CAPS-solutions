

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(sum(2,2), 4);
        assert_eq!(sum(8,2), 10);
    }

    #[test]
    fn subtract_test() {
        assert_eq!(subtract(2,2), 0);
        assert_eq!(subtract(8,2), 6);
        assert_eq!(subtract(2,8), -6);
    }

    #[test]
    fn multiply_test() {
        assert_eq!(multiply(2,2), 4);
        assert_eq!(multiply(-2,-2), 4);
        assert_eq!(multiply(2,-2), -4);
        assert_eq!(multiply(2,0), 0);
    }

    #[test]
    #[ignore] //-gb- faz o cargo ignorar este teste
    fn divide_test() {
        assert_eq!(divide(8,2), 4);
        assert_eq!(divide(-8,-2), 4);
        assert_eq!(divide(8,-2), -4);
        assert_eq!(divide(0,2), 0);
    }
}
