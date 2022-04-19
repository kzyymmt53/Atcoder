use proconio::input;

fn main() {

    input! { 
        n: i64,
        k: i64,
    }

    input! {
        a: [i64; n],
    }

    let mut visit: Vec<i64> = vec![-1; n as usize + 1];


    let mut start: i64 = 1;
    let mut count: i64 = 0;
    let mut flag = false;
    for _ in 0..k {
        if visit[start as usize] != -1 {
            flag = true;
            break;
        }
        visit[start as usize] = count;
        count += 1;
        start = a[start as usize - 1];
         
    }

    let mut ans = start;
    if flag {
        let aaa = (k - visit[start as usize]) % ( count - visit[start as usize]);  
        for _ in 0..aaa {
            ans = a[ans as usize - 1];  
        }
        println!("{}", ans);
    }else {
        println!("{}", start);
    }

}

