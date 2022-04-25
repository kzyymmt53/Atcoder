use proconio::input;
use std::collections::VecDeque;

fn main() {

    input! { 
        n: usize,
        m: i64,
    }

    let mut v: Vec<Vec<i64>> = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
        }
        v[a as usize - 1].push(b - 1);
        v[b as usize - 1].push(a - 1);
    }

    let mut que: VecDeque<i64> = VecDeque::new();
    que.push_back(0); 
    let mut ans: Vec<i64> = vec![120; n];
    ans[0] = 0;

    while !que.is_empty() {
        let point: i64 = que.pop_front().unwrap();
        for i in &v[point as usize] {
            if ans[*i as usize] == 120 {
                ans[*i as usize] = ans[point as usize] + 1; 
                que.push_back(*i);
            }
        }
    }

    for i in 0..n {
        if ans[i] > 120 {
            println!("120");
        }else{
            println!("{}", ans[i]);
        }
    }

}

