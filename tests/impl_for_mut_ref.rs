#[test]

fn test() {
    pub trait Get {
        type S;

        fn get(self) -> Self::S;
    }

    pub trait Update {
        type T;

        fn update(
            self,
            x: Self::T,
        );
    }

    struct A(usize);

    impl<'a> Get for &'a mut A {
        type S = &'a mut usize;

        fn get(self) -> Self::S { &mut self.0 }
    }

    impl Update for &mut A {
        type T = A;

        fn update(
            self,
            x: A,
        ) {
            *self = x
        }
    }

    let mut a = A(2);

    a.update(A(3));

    println!("{}", a.get());

    *a.get() = 5;

    println!("{}", a.get());
}
