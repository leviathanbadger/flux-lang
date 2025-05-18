use quickcheck::quickcheck;

quickcheck! {
    fn addition_commutes(x: i32, y: i32) -> bool {
        x.wrapping_add(y) == y.wrapping_add(x)
    }
}
