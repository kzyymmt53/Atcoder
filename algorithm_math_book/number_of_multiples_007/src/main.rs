use proconio::input;

fn main() {

    input! { 
        n: i32,
        x: i32,
        y: i32,
    }

    let start;
    if x < y {
        start = x;
    }else{
        start = y;
    }

    let mut ans = 0;
    for i in start..=n {
        if i % x == 0 {
            ans += 1;
        }else if i % y == 0 {
            ans += 1;
        }
        
    }

    println!("{}", ans);

}

