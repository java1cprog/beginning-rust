// It does nothing.
fn main() {
    let one_thousand = 1e3;
    let one_million = 1e6;
    let thirteen_billions_and_half = 13.5e9;
    let twelve_millionths = 12e-6;

    println!(
        "{} {} {} {}",
        one_thousand, one_million, thirteen_billions_and_half, twelve_millionths
    );
}
