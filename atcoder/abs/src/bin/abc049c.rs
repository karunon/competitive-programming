use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let search_str = ["dreamer", "eraser", "dream", "erase"];

    loop {
        let mut n = 0;

        if s.chars().count() == 0 {
            println!("YES");
            return;
        }

        for search in search_str.iter() {
            let mut search_flag = false;
            if s.len() >= search.len() {
                let copy_s: Vec<char> = s.chars().collect();
                let copy_search: Vec<char> = search.chars().collect();

                for i in 0..copy_search.len() {
                    if copy_s[copy_s.len() - 1 - i] != copy_search[copy_search.len() - 1 - i] {
                        break;
                    }
                    if i == copy_search.len() - 1 {
                        search_flag = true;
                        break;
                    }
                }
                if search_flag {
                    s = s[..s.len() - search.len()].to_string();
                    break;
                }
            }
            if !search_flag {
                n += 1;
            }
        }
        if n == search_str.len() {
            println!("NO");
            return;
        }
    }
}
