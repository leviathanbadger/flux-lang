use quickcheck::quickcheck;

quickcheck! {
    fn addition_commutes(x: i32, y: i32) -> bool {
        x + y == y + x
    }
}
