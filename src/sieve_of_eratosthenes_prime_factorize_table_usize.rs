pub fn prime_factorize(size: usize) -> Vec<Vec<(usize, usize)>> {
    let mut factors = vec![vec![]; size];

    let mut v: Vec<_> = (0..size).collect();

    for i in 2..size {
        if !factors[i].is_empty() {
            continue;
        }

        for j in (i..size).step_by(i) {
            let mut e = 0;

            while v[j] % i == 0 {
                e += 1;

                v[j] /= i;
            }

            factors[j].push((i, e));
        }
    }

    factors
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        let a = prime_factorize(20);

        dbg!(
            a,
            vec![
                vec![],
                vec![],
                vec![(2, 1,),],
                vec![(3, 1,),],
                vec![(2, 2,),],
                vec![(5, 1,),],
                vec![(2, 1,), (3, 1,),],
                vec![(7, 1,),],
                vec![(2, 3,),],
                vec![(3, 2,),],
                vec![(2, 1,), (5, 1,),],
                vec![(11, 1,),],
                vec![(2, 2,), (3, 1,),],
                vec![(13, 1,),],
                vec![(2, 1,), (7, 1,),],
                vec![(3, 1,), (5, 1,),],
                vec![(2, 4,),],
                vec![(17, 1,),],
                vec![(2, 1,), (3, 2,),],
                vec![(19, 1,),],
            ]
        );
    }
}
