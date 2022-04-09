use proconio::input;

fn main() {

    input! { 
        n: usize,
        m: usize,
    }

    let mut nodes: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    let mut ans: Vec<i64> = vec![0; n + 1];

    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
        }
        nodes[a as usize].push(b);
        nodes[b as usize].push(a);
    }

    for i in 1..nodes.iter().len() {
        for j in  &nodes[i] {
            if i as i64 > *j {
                ans[i as usize] += 1;
            }
        }
    }
    
    let mut result = 0;
    for i in ans {
        if i == 1 {
            result += 1;
        }
    }
    println!("{}", result);
}

