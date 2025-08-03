use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    let mut ans = 0;
    let mut odd_flag = false;
    loop {
        for i in 0..n {
            if a[i] % 2 == 1 {
                odd_flag = true;
            }
            a[i] /= 2;
        }
        if odd_flag {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
