use proconio::input;

fn main() {

    let mut tscore: i32 = 0;
    let mut hscore: i32 = 0;

    input! {
        n: usize,
    }
    
    for _ in 0..n {
        input! {
            t: String,
            h: String,
        }

        let size: usize;
        if t.len() <= h.len() {
            size = t.len();
        }else{
            size = h.len();
        } 

        let mut flag = 0;
        for i in 1..size {
            if t[i-1..i] < h[i-1..i] {
                hscore += 3;
                flag = 1;
                break;
            }
            if t[i-1..i] > h[i-1..i] {
                tscore += 3;
                flag = 1;
                break;
            }
        }
        if flag == 0 {
            tscore += 1;
            hscore += 1;
        }
    } 
    
    println!("{} {}", tscore, hscore);

}

