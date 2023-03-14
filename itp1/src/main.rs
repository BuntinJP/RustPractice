use std::io::{self, BufRead};
fn main() {
    let st = io::stdin();
    let mut l = st.lock().lines();
    let s = l.next().unwrap().unwrap();
    let p = l.next().unwrap().unwrap();
    let mut s2 = s.clone();
    s2.push_str(&s);
    println!("{}", if s2.contains(&p) { "Yes" } else { "No" });
}
