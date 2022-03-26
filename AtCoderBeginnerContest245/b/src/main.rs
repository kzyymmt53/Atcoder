use proconio::input;

fn main() {
    input! {
        n: usize, 
    }
  
    let mut v: Vec<i32> = Vec::new();
   
    for _ in 0..n {
        input! {
            a: i32,
        }
        v.push(a);   
    }

    v.sort();
  
    let mut pre = 0;
    let mut flag = 0;
    for i in 0..n {
        if i == 0 && v[i] != 0 {
            println!("0");
            flag = 1;
            break;
        }else if i == 0 && v[i] == 0 {
            pre = 0;
        }else{
            if v[i] - pre == 1 || v[i] - pre == 0 {
                pre = v[i];
            }else{
                println!("{}", pre + 1);
                flag = 1;
                break;
            }
        }
    } 
    if flag == 0 {
        println!("{}", pre + 1);
    }
}

