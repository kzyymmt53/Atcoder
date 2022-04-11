use std::collections::VecDeque;
use proconio::input;

fn main() {

    input! { 
        r: usize,
        c: usize,
    }

    input! {
        s: (i64, i64),
    }

    input! {
        g: (i64, i64),
    }

    let mut v: Vec<Vec<char>> = Vec::new();
    let mut result: Vec<Vec<i64>> = vec![vec![-1; c]; r];
    
    for _ in 0..r {
        input! {
            d: String,
        }
        let mut temp: Vec<char> = Vec::new();
        for a in d.chars() {
            temp.push(a);
        }
        v.push(temp);
    }


    bfs(&mut result, &v, s, r, c);

    println!("{}", result[g.0 as usize - 1][g.1 as usize - 1]);

}

fn bfs(result: &mut Vec<Vec<i64>>, v: &Vec<Vec<char>>, s: (i64, i64),  r:usize, c:usize) {
    let mut deque: VecDeque<(i64, i64)> = VecDeque::new();
    result[s.0 as usize - 1][s.1 as usize - 1] = 0;
    deque.push_back(s);

    while !deque.is_empty() {
        let current = deque.pop_front().unwrap();
        
        if s.0 - 2 > -1 {
            if v[current.0 as usize - 2][current.1 as usize - 1] == '.' && result[current.0 as usize - 2][current.1 as usize - 1] == -1 {
                result[current.0 as usize - 2][current.1 as usize - 1] = result[current.0 as usize - 1][current.1 as usize - 1] + 1;
                deque.push_back((current.0 - 1, current.1));
            }
        }

        if s.1 - 2 > -1 {
            if v[current.0 as usize - 1][current.1 as usize - 2] == '.' && result[current.0 as usize - 1][current.1 as usize - 2] == -1 {
                result[current.0 as usize - 1][current.1 as usize - 2] = result[current.0 as usize - 1][current.1 as usize - 1] + 1;
                deque.push_back((current.0, current.1 - 1));
            }
        }

        if s.0 < c as i64 {
            if v[current.0 as usize][current.1 as usize - 1] == '.' && result[current.0 as usize][current.1 as usize - 1] == -1 {
                result[current.0 as usize ][current.1 as usize - 1] = result[current.0 as usize - 1][current.1 as usize - 1] + 1;
                deque.push_back((current.0 + 1, current.1));
            }
        }

        if s.1 < r as i64{
            if v[current.0 as usize - 1][current.1 as usize] == '.' && result[current.0 as usize - 1][current.1 as usize] == -1 {
                result[current.0 as usize - 1][current.1 as usize] = result[current.0 as usize - 1][current.1 as usize - 1] + 1;
                deque.push_back((current.0, current.1 + 1));
            }
        }

    }

}

