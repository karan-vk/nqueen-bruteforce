// use itertools::Itertools;

// fn is_solution(perm: Vec<i32>) -> bool {
//     for i in (0..perm.len() as i32).into_iter().combinations(2) {
//         let x = i.first().unwrap().clone();
//         let y = i.last().unwrap().clone();
//         println!("{} {} ", x, y);
//         let a = ((x - y) as i32).abs();
//         let b = (perm[x as usize] - perm[y as usize]).abs();
//         if a == b {
//             return false;
//         }
//     }
//     return true;
// }

// fn can_sol_ext(perm: &mut Vec<i32>) -> bool {
//     let i = (perm.len() - 1) as i32;
//     for j in 0..i {
//         if i - j == (perm[i as usize] - perm[j as usize]).abs() {
//             return false;
//         }
//     }
//     return true;
// }

// fn extend(perm: &mut Vec<i32>, n: i32, store: &mut Vec<Vec<i32>>) {
//     if perm.clone().len() == n as usize {
//         // println!("{:?}", perm);
//         store.push(perm.clone());
//         return;
//     }
//     for i in 0..n {
//         if !perm.contains(&i) {
//             perm.push(i);
//             if can_sol_ext(perm) {
//                 extend(perm, n, store);
//             }
//             perm.pop();
//         }
//     }
// }

// fn main() {
//     const N: i32 = 5;
//     // let x: Vec<i32> = (0..N).collect();
//     // for perm in x.clone().into_iter().permutations(N as usize) {
//     //     if is_solution(perm.to_vec()) {
//     //         println!("{:?}", perm);
//     //     }
//     // }
//     let mut store: Vec<Vec<i32>> = Vec::new();
//     extend(&mut Vec::new(), N, &mut store);
//     for i in store {
//         println!("{:?}", i);
//     }
//     // println!("{}", store);
// }
// #[cfg(test)]
// mod tests {
//     use crate::is_solution;

//     #[test]
//     fn it_works() {
//         let perm = vec![0, 1, 2, 3];
//         assert_eq!(is_solution(perm), false);
//         assert_eq!(
//             is_solution(vec![
//                 0, 2, 4, 10, 17, 11, 14, 12, 18, 3, 1, 7, 5, 16, 13, 19, 8, 6, 15, 9
//             ]),
//             true
//         );
//     }
// }

const N: usize = 20;

fn work(mut board: &mut [[bool; N]; N], row: usize, mut count: &mut i64) {
    if row == N {
        *count += 1;
        println!("Solution #{}:\n", *count);
        // for r in board.iter() {
        //     println!(
        //         "{}",
        //         r.iter()
        //             .map(|&x| if x { "Q" } else { "." }.to_string())
        //             .collect::<Vec<String>>()
        //             .join(" ")
        //     )
        // }
        // println!("");
        return;
    }
    for i in 0..N {
        let mut ok: bool = true;
        for j in 0..row {
            if board[j][i]
                || i + j >= row && board[j][i + j - row]
                || i + row < N + j && board[j][i + row - j]
            {
                ok = false
            }
        }
        if ok {
            board[row][i] = true;
            work(&mut board, row + 1, &mut count);
            board[row][i] = false;
        }
    }
}

fn main() {
    let mut board: [[bool; N]; N] = [[false; N]; N];
    let mut count: i64 = 0;
    work(&mut board, 0, &mut count);
}
