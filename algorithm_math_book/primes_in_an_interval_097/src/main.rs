use proconio::input;

fn main() {

    input! { 
        l: usize,
        r: usize,
    }

    let mut v: Vec<bool> = vec![true; r - l + 1];

    if l == 1 {
        v[0] = false;
    }
    let mut ans = 0;
    //LからRまでの素数でないものを確認
    for i in 2..=r {
        let min_val = (l + i - 1) / i * i;
        for j in (min_val..=r).step_by(i) {
            if i == j {
                continue;
            }
            v[j - l] = false;
        }

        if i * i > r {
            break;
        }
       
    }


    for i in v {
        if i {
            ans += 1;
        }
    }

    println!("{}", ans);

}

