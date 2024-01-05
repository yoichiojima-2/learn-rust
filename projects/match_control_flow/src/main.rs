enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_centes(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = value_in_centes(Coin::Penny);
    let nickel = value_in_centes(Coin::Nickel);
    let dime = value_in_centes(Coin::Dime);
    let quarter = value_in_centes(Coin::Quarter);
    println!("penny = {}, nickel = {}, dime = {}, quarter = {}", penny, nickel, dime, quarter);
}
