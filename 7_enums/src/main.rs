
mod poker_enums;
mod ip_enum;
mod message;

use ip_enum::*;
use std::io::SeekFrom;
use poker_enums::*;

use crate::message::Message::{ChangeColor, Move, Quit};

#[allow(unused)]
fn main() {
    //TODO: enum Option<T> very useful
    //There is no NULL/nullptr in Rust to avoid null pointer fault
    let some_value : Option<i32> = Some(12); //Set a type and example value for this var is to be used.
    let none_calue : Option<String> = None;  //Set a type and None for this var is to be used.

    // Example IP
    let four = IpAddrType::V4;
    let six = IpAddrType::V6;
    let home = IpAddr{
        ip_type: IpAddrType::V4,
        ip: String::from("127.0.0.1")
    };
    let loopback = IpAddr{
        ip_type: IpAddrType::V6,
        ip: String::from("::1")
    };
    route(four);
    route(IpAddrType::V6);

    // Example poker
    let card_1 = PokerCard{
        suit: PokerSuit::Diamonds,
        value: 5,
    };

    let card_2 = PokerCard{
        suit: PokerSuit::Clubs,
        value: 9,
    };

    let card_3 = Poker::Hearts(3);
    card_3.show_poker();

    //Message
    let m1 = Quit;
    let m2 = Move{x:1,y:1};
    let m3 = ChangeColor(255, 255, 0);
}

