use std::collections::VecDeque;
use proconio::input;

fn main() {

    input! { 
        n: usize,
        m: usize,
    }

    let mut v: Vec<Vec<(i64, i64)>> = vec![vec![]; n  + 1];
    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
            c: i64,
        }

        v[a as usize].push((b, c));
        v[b as usize].push((a, c));
    }

    let mut ans: Vec<i64> = vec![9223372036854775807; n + 1];
    ans[0] = 0;
    ans[1] = 0;

    let mut deque: VecDeque<i64> = VecDeque::new();
    deque.push_back(1);

    while !deque.is_empty(){
        let now = deque.pop_front().unwrap();
        for i in &v[now as usize] {
            if ans[i.0 as usize] > i.1 + ans[now as usize] {
                ans[i.0 as usize] = i.1 + ans[now as usize];
                deque.push_back(i.0);
            }
        }
    }
    
    if ans[n] == 9223372036854775807 {
        println!("-1");
    }else{
        println!("{}", ans[n]);
    }


    

}

