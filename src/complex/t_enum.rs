pub fn test_enum() {
    create_enum();
}

// 初始化枚举
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

// 创建枚举
fn create_enum() {
    let heart = PokerSuit::Hearts;
    print_enum(heart);

    let c1 = PokerCard::Hearts(5);
    print_enum_card(c1);

}

// 输出枚举
fn print_enum(card: PokerSuit) {
    println!("{:?}", card);
}
fn print_enum_card(card: PokerCard) {
    println!("{:?}", card);
}





