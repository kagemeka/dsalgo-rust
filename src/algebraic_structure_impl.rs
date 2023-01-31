use crate::{
    binary_function::*,
    group_theory_id::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct GroupApprox<S, I>(std::marker::PhantomData<(S, I)>);

/// ((usize, usize), min)

impl BinaryOp for GroupApprox<(usize, usize), Min> {
    type S = (usize, usize);

    fn op(
        lhs: Self::S,
        rhs: Self::S,
    ) -> Self::S {
        lhs.min(rhs)
    }
}

impl Idempotence for GroupApprox<(usize, usize), Min> {}

impl Commutative for GroupApprox<(usize, usize), Min> {}

impl Associative for GroupApprox<(usize, usize), Min> {}

impl Identity for GroupApprox<(usize, usize), Min> {
    fn e() -> Self::S { (std::usize::MAX, std::usize::MAX) }
}

/// (usize, min)

impl BinaryOp for GroupApprox<usize, Min> {
    type S = usize;

    fn op(
        lhs: usize,
        rhs: usize,
    ) -> usize {
        lhs.min(rhs)
    }
}

impl Idempotence for GroupApprox<usize, Min> {}

impl Commutative for GroupApprox<usize, Min> {}

impl Associative for GroupApprox<usize, Min> {}

impl Identity for GroupApprox<usize, Min> {
    fn e() -> Self::S { std::usize::MAX }
}

/// (usize, +)

impl BinaryOp for GroupApprox<usize, Additive> {
    type S = usize;

    fn op(
        lhs: usize,
        rhs: usize,
    ) -> usize {
        lhs + rhs
    }
}

impl Associative for GroupApprox<usize, Additive> {}

impl Commutative for GroupApprox<usize, Additive> {}

impl Identity for GroupApprox<usize, Additive> {
    fn e() -> Self::S { 0 }
}

/// (i32, +)

impl BinaryOp for GroupApprox<i32, Additive> {
    type S = i32;

    fn op(
        lhs: i32,
        rhs: i32,
    ) -> i32 {
        lhs + rhs
    }
}

impl Associative for GroupApprox<i32, Additive> {}

impl Commutative for GroupApprox<i32, Additive> {}

impl Identity for GroupApprox<i32, Additive> {
    fn e() -> Self::S { 0 }
}

impl Inverse for GroupApprox<i32, Additive> {
    fn inv(x: i32) -> i32 { -x }
}

/// (u64, +)

impl BinaryOp for GroupApprox<u64, Additive> {
    type S = u64;

    fn op(
        lhs: Self::S,
        rhs: Self::S,
    ) -> Self::S {
        lhs + rhs
    }
}

impl Commutative for GroupApprox<u64, Additive> {}

impl Associative for GroupApprox<u64, Additive> {}

impl Identity for GroupApprox<u64, Additive> {
    fn e() -> Self::S { 0 }
}

/// (i64, +)

impl BinaryOp for GroupApprox<i64, Additive> {
    type S = i64;

    fn op(
        lhs: Self::S,
        rhs: Self::S,
    ) -> Self::S {
        lhs + rhs
    }
}

impl Commutative for GroupApprox<i64, Additive> {}

impl Associative for GroupApprox<i64, Additive> {}

impl Identity for GroupApprox<i64, Additive> {
    fn e() -> Self::S { 0 }
}

impl Inverse for GroupApprox<i64, Additive> {
    fn inv(x: i64) -> i64 { -x }
}

/// (u64, min)

impl BinaryOp for GroupApprox<u64, Min> {
    type S = u64;

    fn op(
        lhs: Self::S,
        rhs: Self::S,
    ) -> Self::S {
        lhs.min(rhs)
    }
}

impl Idempotence for GroupApprox<u64, Min> {}

impl Commutative for GroupApprox<u64, Min> {}

impl Associative for GroupApprox<u64, Min> {}

impl Identity for GroupApprox<u64, Min> {
    fn e() -> Self::S { std::u64::MAX }
}
