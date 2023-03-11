// truncated division
pub fn div<T: std::ops::Div<Output = T>>(a: T, b: T) -> T {
    a / b
}
#[test]
fn test() {
    let cases = [(5, 2, 2), (5, -2, -2), (-5, 2, -2), (-5, -2, 2)];
    for (a, b, ans) in cases {
        assert_eq!(div(a, b), ans);
    }
}
