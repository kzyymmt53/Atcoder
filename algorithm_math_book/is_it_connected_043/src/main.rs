use proconio::input;

fn main() {

    input! { 
        n: usize,
        m: usize,
    }

    let mut v: Vec<bool> = vec![false; n + 1];
    let mut nodes: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    v[0] = true;

    for _ in 0..m {
        input!{
            a: i64,
            b: i64,
        }

        nodes[a as usize].push(b);
        nodes[b as usize].push(a);
    }

    dfs(&nodes, 1, &mut v);


    let mut ans = true;
    for d in v{
        if !d {
            ans = false;
            break;
        }
    }

    if ans {
        println!("The graph is connected.");
    }else{
        println!("The graph is not connected.");
    }

}

fn dfs(nodes: &Vec<Vec<i64>>, target: usize, v: &mut Vec<bool>){
    v[target] = true;
    for d in &nodes[target] {
        if !v[*d as usize] {
            dfs(nodes, *d as usize, v);
        }
            
    }
}

