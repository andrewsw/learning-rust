fn main() {
    for n in range(1i,101) {
        println!("{}", fizzbuzz(n));
    }
}

fn fizzbuzz(n: int) -> String {
        match (n % 3, n % 5) {
            (0, 0) => format!("fizzbuzz"),
            (0, _) => format!("fizz"),
            (_, 0) => format!("buzz"),
            _      => format!("{}", n)
        }

}