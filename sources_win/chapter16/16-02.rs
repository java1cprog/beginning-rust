/* It prints:
e*/
fn print_nth_char(s: &str, n: u32) {
    let mut iter: std::str::Chars = s.chars();
    let n_item = n-1;
    let mut index = 0;
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => 
             if n_item == index
            {
                println!("{} ", c);
            },
            None => {
                break;
            }
        }
        index += 1;
    }
}

fn main() {
    let string = "€èe";
    print_nth_char(string, 1);
    print_nth_char(string, 2);
    print_nth_char(string, 3);
}

