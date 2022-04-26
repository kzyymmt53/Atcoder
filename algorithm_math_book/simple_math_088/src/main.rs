use proconio::input;

fn main() {

    input! { 
        mut a: u64,
        mut b: u64,
        mut c: u64,
    }

    let m: u64 = 998244353;
    
    let aa: u64 = a * (a + 1) / 2 % m;
    let bb: u64 = b * (b + 1) / 2 % m; 
    let cc: u64 = c * (c + 1) / 2 % m;

    

    println!("{}", aa * bb % m * cc % m);

}

