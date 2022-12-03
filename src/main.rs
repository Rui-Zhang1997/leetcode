mod solns;
use solns::longest_unrepeating;

fn main() {
    let v = String::from("a 1 a");
    println!("{}: max_substring_length : {}", v, longest_unrepeating::soln(&v));
}
