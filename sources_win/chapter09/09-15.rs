// It prints 73 73 173.
fn main() {
    fn f1() -> i32 { 4.5; "abc"; 73i32 }
    fn f2() -> i32 { 4.5; "abc"; 73 }
    fn f3() -> i32 { 4.5; "abc"; 73 + 100 }

    println!("{}", f1());
    println!("{}", f2());
    println!("{}", f3());
}
