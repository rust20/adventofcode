pub(crate) fn sub(a: i32, b: i32) -> i32 {
    let mut a = a;
    for _ in 0..b {
        a -=1;
    }
    a
}
