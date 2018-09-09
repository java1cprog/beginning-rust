// ILLEGAL
fn main() {
    match contin {
        Continent::Europe => let a = 7;,
        Continent::Asia => let a = 7,
        Continent::Africa => fn aaa() {},
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }
}
/*
#[allow(dead_code)]
fn main() {
    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let contin = Continent::Asia;
    match contin {
     //   Continent::Europe => let a = 7;,
        //Continent::Asia => let a = 7,
    //    Continent::Africa => fn aaa() {},
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
        _ => {},
    }
}

*/
