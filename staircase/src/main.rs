fn staircase(n: i32) {
    for i in 0..n + 1 {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        if hashes.contains("#") {
            println!("{}{}", spaces, hashes);
        }
    }
}

fn main() {
    staircase(4);
}
