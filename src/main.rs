use itertools::Itertools;

fn is_solution(perm: Vec<i32>, n: i32) -> bool {
    for i in (0..n).into_iter().combinations(2) {
        let x = i.first().unwrap().clone();
        let y = i.last().unwrap().clone();
        let a = ((x - y) as i32).abs();
        let b = (perm[x as usize] - perm[y as usize]).abs();
        if a == b {
            return false;
        }
    }
    return true;
}

fn main() {
    const N: i32 = 4;
    let x: Vec<i32> = (0..N).collect();
    for perm in x.into_iter().permutations(N as usize) {
        if is_solution(perm.to_vec(), N) {
            println!("{:?}", perm);
        }
    }
}
