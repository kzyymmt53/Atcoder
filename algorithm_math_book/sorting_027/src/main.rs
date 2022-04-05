use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    input! {
        mut a: [i64; n ],
    }

    let mut c:Vec<i64> = vec![0; n];

    merge_sort(0, n , &mut a, &mut c);

    let mut flag = 0;
    for d in a {
        if flag == 0 {
            print!("{}", d);
        }else{
            print!(" {}", d);
        }
        flag = 1;
    }

    println!("");

}

fn merge_sort(l: usize, r: usize, a: &mut Vec<i64>, c: &mut Vec<i64>){
    if r - l == 1 {
        return;
    }

    let m = (l + r) / 2;
    merge_sort(l, m, a, c);
    merge_sort(m, r, a, c);

    let mut c1: usize = l;
    let mut c2: usize = m;
    let mut cnt = 0;

    while c1 != m || c2 != r {
        if c1 == m {
            c[cnt] = a[c2]; 
            c2 += 1;
        }else if c2 == r {
            c[cnt] = a[c1];
            c1 += 1;
        }else{
            if a[c1] > a[c2] {
                c[cnt] = a[c2];
                c2 += 1;
            }else{
                c[cnt] = a[c1];
                c1 += 1;
            }
        }
        cnt += 1;
    }

    for i in 0..cnt {
        a[l + i ] = c[i];
    }
}
