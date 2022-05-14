use proconio::input;

fn main() {

    input! { 
        n: usize,
        l: i64,
        k: i64,
        a: [i64; n],
    }

    let mut left: i64 = -1;
    let mut right: i64 = l + 1;

    while right  - left > 1 {
        let mid = left + (right - left) / 2;
        if search(mid, &a, n, l, k) {
            left = mid;
        }else{
            right = mid;
        }


    }

    println!("{}", left);

}

fn search(mid: i64, a: &Vec<i64>, n: usize, l: i64, k: i64) -> bool {
    let mut count = 0;
    let mut start = 0;
    for i in 0..n {
        if a[i] - start >= mid && mid <= l - a[i] {
            count+= 1;
            start = a[i];
        }
    }

    if k <= count {
        return true;
    }else{
        return false;
    }
}
