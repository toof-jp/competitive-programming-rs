use proconio::input;

fn main() {
    input! {
        mut s: String
    }

    for _ in 0..=10 {
        if is_palindrome(&s) {
            println!("Yes");
            return ();
        }
        s = String::from("0") + &s;
    }
    println!("No");
}

fn is_palindrome(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let mut i: usize = 0;
    while i < s.len() {
        if s[i] != s[s.len() - 1 - i] {
            return false
        }
        i += 1;
    }
    true
}
