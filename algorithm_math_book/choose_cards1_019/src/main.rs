use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    input! {
        a: [i64; n],
    }


    let mut v: Vec<i64> = vec![0; 3];

    for d in a {
        if d == 1 {
            v[0] += 1;
        }else if d == 2 {
            v[1] += 1;
        }else{
            v[2] += 1;
        }
    }

    //それぞれの色のカードから2枚とった時のパターンを計算
    let ans = v[0] * (v[0] - 1) / 2 + v[1] * (v[1] - 1) / 2 + v[2] * (v[2] - 1) / 2;


    println!("{}", ans);

}

