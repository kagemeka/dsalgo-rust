#[test]

fn test() {
    pub struct A<F: Fn()> {
        f: F,
    }

    let f = || println!("1");

    let a = A { f: &f };

    (a.f)();
}
