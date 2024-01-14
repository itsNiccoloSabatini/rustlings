// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.


fn test_of_test (value:i32)->bool{
    if value < 0 {
        false
    }else{
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::test_of_test;

    #[test]
    fn you_can_assert1() {
        let a: i32 = 1980;
        assert_eq!(test_of_test(a), true);
    }

    fn you_can_assert2() {
        let b: i32 = -321;
        assert_eq!(test_of_test(b), true);
        
    }
}
