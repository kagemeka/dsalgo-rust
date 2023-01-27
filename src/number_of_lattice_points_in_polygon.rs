use crate::{
    number_of_lattice_points_on_segment::segment_lattice_points,
    polygon_area_2_times_2d::polygon_area_x2,
};

/// by Pick's Theorem
/// S = i + 2/b - 1
/// i := interior lattice points
/// b := boundary lattice points
/// compute i + b (include boundary)

pub fn polygon_lattice_points(a: &[(i64, i64)]) -> i64 {
    let n = a.len();

    let s2 = polygon_area_x2(&a);

    let mut b = 0;

    for i in 0..n {
        let j = (i + 1) % n;

        b += segment_lattice_points(a[i].0, a[i].1, a[j].0, a[j].1) - 1;
    }

    let i = (s2 + 2 - b) >> 1;

    i + b
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test() {
        // ref: https://twitter.com/e869120/status/1393753066331992065/photo/3
        let cases =
            vec![(vec![(1, 3), (2, 1), (5, 2), (8, 3), (5, 5), (3, 4)], 18)];

        for (a, ans) in cases {
            assert_eq!(polygon_lattice_points(&a), ans);
        }
    }

    #[test]

    fn test_atcoder_typical90_41() {
        use crate::convex_hull_monotone_chain::convex_hull;

        let cases = vec![
            (vec![(1, 4), (6, 1), (5, 8)], 17),
            (vec![(2, 2), (2, 3), (3, 2)], 0),
            (vec![(2, 39), (39, 35), (17, 5)], 599),
            (
                vec![
                    (72, 7),
                    (54, 25),
                    (97, 48),
                    (37, 47),
                    (34, 54),
                    (4, 16),
                    (62, 1),
                    (59, 22),
                    (99, 73),
                    (34, 75),
                ],
                4828,
            ),
            (
                vec![
                    (878317816, 654163251),
                    (686185971, 65193664),
                    (421988001, 893301255),
                    (337790787, 848308131),
                    (116633641, 453711858),
                    (147679897, 275450390),
                    (871549713, 368160131),
                    (945135251, 515070794),
                    (113677189, 553747963),
                    (648722370, 798825746),
                    (334960984, 163211483),
                    (477414168, 849868430),
                    (46724716, 593116536),
                    (424597820, 84043071),
                    (456749260, 981436379),
                    (167906984, 546584517),
                    (187306934, 201207913),
                    (535850448, 43428774),
                    (602081737, 111568378),
                    (607467836, 80430906),
                    (965538187, 537789555),
                    (69199019, 485172741),
                    (267885487, 934316143),
                    (883812229, 276272851),
                    (507976072, 19708905),
                    (951100460, 639017801),
                    (43859603, 556279043),
                    (300658736, 79240016),
                    (231304846, 220059094),
                    (854667690, 399502355),
                ],
                607281204170558988_i64,
            ),
        ];

        for (a, ans) in cases {
            let n = a.len();

            let a = convex_hull(&a);

            assert_eq!(polygon_lattice_points(&a) - n as i64, ans);
        }
    }
}
