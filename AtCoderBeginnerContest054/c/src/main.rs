use proconio::input;
use itertools::Itertools;
use proconio::marker::*;

fn main() {

    input! { 
        n: usize,
        m: usize,
    }

    input! {
            a: [(Usize1,Usize1);m],
    }
   


    let mut result = 0;
    for perm in (0..n).permutations(n) {
        let mut ans = true;
        for i in 0..n-1 {
            if i == 0 && perm[i] != 0 {
                ans = false;
                break;
            }
            if !a.contains(&(perm[i], perm[i + 1])) && !a.contains(&(perm[i+1], perm[i])){
                ans = false;
                break;

            }
        }
        if ans {
            result += 1;
        }
    }

    println!("{}", result);

}

