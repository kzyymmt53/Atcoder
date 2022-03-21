use proconio::input;

fn main() {
    input! {
         n: String,
    }

    let target: [&str; 4] = ["maerd","remaerd","esare","resare"];
    let mut start_index: usize = 0;
    let v: Vec<char> = n.chars().rev().collect();
    let v_size = n.chars().rev().count();

    loop {
        if start_index >= v_size {
            println!("YES");
            break;
        }
        let mut flag = 0;
        for i in target.iter() {
           let target_word_list: Vec<char> = i.chars().collect();
           let target_word_list_length_usize: usize = i.chars().count();

           if target_word_list_length_usize + start_index > v_size {
               continue;
           }

           if target_word_list[0..target_word_list_length_usize] == v[start_index..(target_word_list_length_usize + start_index)] {
               start_index = target_word_list_length_usize + start_index;
               flag = 1;
               break;
           }
        }
        if flag == 0 {
            println!("NO");
            break;
        }

    }
}
