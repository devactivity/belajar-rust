enum Coin {
    Head,
    Flag(Asia),
}


#[derive(Debug)]
enum Asia {
    Indonesia,
    Malaysia,
    Singapura,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match coin {
    //     Coin::Head => {
    //         println!("You are lucky!");
    //         1
    //     },
    //     Coin::Flag => 5,
    // }

    let mut count: u8 = 0;
    if let Coin::Flag(location) = coin {
        println!("Location from {:?}", location);
        5
    } else {
        count += 1;
        10
    }

    // match coin {
    //     Coin::Head => 1,
    //     Coin::Flag(state) => {
    //         println!("Country from {:?}", state);
    //         25
    //     }
    // }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x { // exhaustive
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Pattern Matching");
    println!("{} coin", value_in_cents(Coin::Flag(Asia::Indonesia)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => buy_property(),
        7 => buy_ticket(),
        // other => go_to_jail(other),
        _ => (),
    }

    // let config_max: Option<u8> = Some(3);
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is {}", max);
    }


}

fn test5() {}
fn reroll() {}
fn buy_ticket() {}
fn buy_property() {}
fn go_to_jail(number: u8) {}
