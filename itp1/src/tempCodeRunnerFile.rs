fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "0 0" {
            break;
        }
        let conditions: Vec<usize> = input
            .trim()
            .split_whitespace()
            .flat_map(str::parse)
            .collect();
        let n = conditions[0];
        let x = conditions[1];
        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if i + j + k == x && i < j && j <= k {
                        count += 1;
                    }
                }
            }
        }
        println!("{}", count);
    }
}
