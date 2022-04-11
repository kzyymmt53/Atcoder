use proconio::input;

fn main() {

    input! { 
        n: usize,
        m: usize,
    }


    let mut v: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    let mut ans: Vec<i64> = vec![-1; n + 1];

    for _ in 0..m {
        input! {
            a: i64,
            b: i64,
        }

        v[a as usize].push(b);
        v[b as usize].push(a);

    }

    let mut bipart = true;
    for i in 1..n + 1 {
        if ans[i] != -1 {
            continue;
        }
        if !dfs(&mut ans, &v, i, 0){
            bipart= false;
        }
    }


    if bipart {
        println!("Yes");
    }else{
        println!("No");
    }

}

fn dfs(ans: &mut Vec<i64>, v: &Vec<Vec<i64>>, target: usize, n: i64) -> bool{
   ans[target] = n;
   for d in &v[target] {
       if ans[*d as usize] != -1 { 
           if ans[target] ==  ans[*d as usize] {
               return false;
           }else{
               continue;
           }
       }
       if !dfs(ans, v, *d as usize, 1 - n) {
           return false;
       }
   }

   return true;

}

