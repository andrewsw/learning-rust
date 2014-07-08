fn main() {
    for n in range(1i,101) {
        match (n % 3, n % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _      => println!("{}", n)
        }
    }
}