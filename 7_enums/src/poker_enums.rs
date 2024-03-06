
pub enum PokerSuit{
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}
// Another way - valued enum
#[derive(Debug)]
pub enum Poker{
    Diamonds(u8),
    Clubs(u8),
    Hearts(u8),
    Spades(u8),
}

impl Poker {
    pub fn show_poker(self){
        println!("{:#?}", self);
    }
}
// One way to describe a card - struct
pub struct PokerCard{
    pub(crate) suit: PokerSuit,
    pub(crate) value :u8,
}