#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => Card::Two,
        }
    }
}

impl From<usize> for Card {
    fn from(n: usize) -> Self {
        match n {
            0 => Card::Ace,
            1 => Card::King,
            2 => Card::Queen,
            3 => Card::Jack,
            4 => Card::Ten,
            5 => Card::Nine,
            6 => Card::Eight,
            7 => Card::Seven,
            8 => Card::Six,
            9 => Card::Five,
            10 => Card::Four,
            11 => Card::Three,
            12 => Card::Two,
            _ => Card::Two,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cards<const N: usize> {
    cards: [Card; N],
}

impl<const N: usize> Cards<N> {
    fn new(mut cards: [Card; N]) -> Self {
        cards.sort();
        Self { cards }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind(Card),
    FourOfAKind(Card, Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card, Cards<2>),
    TwoPair(Cards<2>, Card),
    OnePair(Card, Cards<3>),
    HighCard(Cards<5>),
}

impl HandType {
    fn kind_cmp(&self, other: &HandType) -> std::cmp::Ordering {
        match (self, other) {
            (Self::FiveOfAKind(..), Self::FiveOfAKind(..)) => std::cmp::Ordering::Equal,
            (Self::FiveOfAKind(..), _) => std::cmp::Ordering::Less,
            (_, Self::FiveOfAKind(..)) => std::cmp::Ordering::Greater,

            (Self::FourOfAKind(..), Self::FourOfAKind(..)) => std::cmp::Ordering::Equal,
            (Self::FourOfAKind(..), _) => std::cmp::Ordering::Less,
            (_, Self::FourOfAKind(..)) => std::cmp::Ordering::Greater,

            (Self::FullHouse(..), Self::FullHouse(..)) => std::cmp::Ordering::Equal,
            (Self::FullHouse(..), _) => std::cmp::Ordering::Less,
            (_, Self::FullHouse(..)) => std::cmp::Ordering::Greater,

            (Self::ThreeOfAKind(..), Self::ThreeOfAKind(..)) => std::cmp::Ordering::Equal,
            (Self::ThreeOfAKind(..), _) => std::cmp::Ordering::Less,
            (_, Self::ThreeOfAKind(..)) => std::cmp::Ordering::Greater,

            (Self::TwoPair(..), Self::TwoPair(..)) => std::cmp::Ordering::Equal,
            (Self::TwoPair(..), _) => std::cmp::Ordering::Less,
            (_, Self::TwoPair(..)) => std::cmp::Ordering::Greater,

            (Self::OnePair(..), Self::OnePair(..)) => std::cmp::Ordering::Equal,
            (Self::OnePair(..), _) => std::cmp::Ordering::Less,
            (_, Self::OnePair(..)) => std::cmp::Ordering::Greater,

            (Self::HighCard(..), Self::HighCard(..)) => std::cmp::Ordering::Equal,
        }
    }
}

impl From<Vec<Card>> for HandType {
    fn from(hand: Vec<Card>) -> Self {
        let counts = hand.iter().fold([0u8; 13], |mut counts, card| {
            counts[*card as usize] += 1;
            counts
        });
        let mut single = None;
        let mut quad = None;
        let mut triple = None;
        let mut double = None;
        let mut singles = [Card::Two; 5];
        let mut singles_len = 0;
        let mut pairs = [Card::Two; 2];
        let mut pairs_len = 0;
        for (card, &count) in counts.iter().enumerate() {
            match count {
                5 => return Self::FiveOfAKind(card.into()),
                4 => {
                    if let Some(single) = single {
                        return Self::FourOfAKind(card.into(), single);
                    }
                    quad = Some(card.into());
                }
                3 => {
                    if let Some(double) = double {
                        return Self::FullHouse(card.into(), double);
                    }
                    if singles_len == 2 {
                        return Self::ThreeOfAKind(
                            card.into(),
                            Cards::new([singles[0], singles[1]]),
                        );
                    }
                    triple = Some(card.into());
                }
                2 => {
                    if let Some(triple) = triple {
                        return Self::FullHouse(triple, card.into());
                    }
                    if let Some(double) = double {
                        if let Some(single) = single {
                            return Self::TwoPair(Cards::new([double, card.into()]), single);
                        }
                    }
                    if singles_len == 3 {
                        return Self::OnePair(
                            card.into(),
                            Cards::new([singles[0], singles[1], singles[2]]),
                        );
                    }
                    double = Some(card.into());
                    pairs[pairs_len] = card.into();
                    pairs_len += 1;
                }
                1 => {
                    if let Some(quad) = quad {
                        return Self::FourOfAKind(quad, card.into());
                    }
                    if let Some(triple) = triple {
                        if let Some(single) = single {
                            return Self::ThreeOfAKind(triple, Cards::new([single, card.into()]));
                        }
                    }
                    if pairs_len == 2 {
                        return Self::TwoPair(Cards::new(pairs), card.into());
                    }
                    if let Some(double) = double {
                        if singles_len == 2 {
                            return Self::OnePair(
                                double,
                                Cards::new([singles[0], singles[1], card.into()]),
                            );
                        }
                    }
                    single = Some(card.into());
                    singles[singles_len] = card.into();
                    singles_len += 1;
                }
                _ => (),
            }
        }
        Self::HighCard(Cards::new(singles))
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let mut hands = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| c.into())
                .collect::<Vec<Card>>();
            let hand_type: HandType = hand.clone().into();
            let bid: u64 = parts.next().unwrap().parse().unwrap();
            (hand_type, hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|(hand_type1, hand1, _), (hand_type2, hand2, _)| {
        match hand_type1.kind_cmp(hand_type2) {
            std::cmp::Ordering::Equal => hand1.cmp(hand2),
            ord => ord,
        }
    });

    println!("{hands:?}");

    let score = hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) as u64 * bid)
        .sum::<u64>();
    println!("{score}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let mut hands = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand: HandType = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| c.into())
                .collect::<Vec<_>>()
                .into();
            let bid: u64 = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|hand1, hand2| hand1.0.cmp(&hand2.0));

    println!("{hands:?}");

    let score = hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u64 * bid)
        .sum::<u64>();
    println!("{score}");
}
