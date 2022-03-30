use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    let mut w = Vec::new();
    for _ in 0..n {
        input! {
            a: usize,
        }
        let mut v = Vec::new();

        for _ in 0..a {
            input! {
                b: [usize; 2],
            }
            v.push(b);
        }
        w.push(v);
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        //全ての人の発言を調べる
        let mut result = true;
        for (i, v2) in w.iter().enumerate() {
            //ある人の発言を調べる
            for v3 in v2 {
                if bit & 1 << i != 0 {
                    if v3[1] == 1 {
                        if bit & (1 << (v3[0] - 1)) == 0 {
                            result = false;
                        }
                    }else{
                        if bit & (1 << (v3[0] - 1)) != 0 {
                            result = false;
                        }
                    }
                }


            }
        }
        if result {
            let count = (bit as u64).count_ones();
            if ans < count {
                ans = count;
            }
        }

    }
    println!("{}", ans);


}

