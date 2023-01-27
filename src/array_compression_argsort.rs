// TODO:
// array compression with argsort
// 0. a := given array
// 1. arg_a := normal argsort a. O(N\log{N})
// 2. res[arg_a[0]] = 0
// 3. i=[1,n), res[arg_a[i]] = res[arg_a[i-1]]
//                            + (a[arg_a[i]] != a[arg_a[i-1]])

// used in suffix array with doubling and 'sort'.
