use proconio::input;
use std::collections::VecDeque;

fn main() {

    input! { 
        n: i64,
        m: usize,
    }

    let mut v: Vec<i64> = vec![-1; n as usize + 1];
    let mut nodes: Vec<Vec<i64>> = vec![Vec::new(); n as usize + 1];


    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
        }
        nodes[a as usize].push(b);
        nodes[b as usize].push(a);
    }

    v[0] = 0; 
    v[1] = 0; 
    let mut deque: VecDeque<i64> = VecDeque::new();
    deque.push_back(1);
    bfs(&nodes, &mut v, &mut deque);

    for i in 1..n+1 {
        println!("{}", v[i as usize]);
    }


}


fn bfs(nodes: &Vec<Vec<i64>>, v: &mut Vec<i64>,  deque: &mut VecDeque<i64>){
    while !deque.is_empty() {
        let target: usize = deque.pop_front().unwrap() as usize;
        for d in &nodes[target] {
            if v[*d as usize] == -1 {
                v[*d as usize] = v[target] + 1;
                deque.push_back(*d);
            }
        }

    }


}
