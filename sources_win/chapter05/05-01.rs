/* It prints:
This is a sentence in English*/
/*
fn main() {
    let x = ["English", "This", "sentence", "a", "in", "is"];
    print!("{} {} {} {} {} {}",
        x[1], x[5], x[3], x[2], x[4], x[0]);
}
*/
fn main() {
    let x = ["English", "This", "sentence", "a", "in", "is"];
    println!("{} {} {} {} {} {}", x[1], x[5], x[3], x[2], x[4], x[0]);

    let y = ["Hello", ",", " ", "World!"];
    println!("{}{}{}{}", y[0], y[1], y[2], y[3]);
}
