use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i32; 3]; n],
    }

    let mut now = vec![0, 0, 0];

    for i in 0..n {
        let next = &a[i];
        let total_path = (next[1] - now[1]).abs() + (next[2] - now[2]).abs();
        
        if (now[0] - next[0]).abs() % 2 != total_path % 2 || total_path > next[0] - now[0] {
            println!("No");
            return;
        }
        now[0] = next[0];
        now[1] = next[1];
        now[2] = next[2];
    }

    println!("Yes");
}
