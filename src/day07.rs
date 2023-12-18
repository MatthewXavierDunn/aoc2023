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

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum CardJoker {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl CardJoker {
    fn is_joker(&self) -> bool {
        match self {
            Self::Joker => true,
            _ => false,
        }
    }
}

impl From<char> for CardJoker {
    fn from(c: char) -> Self {
        match c {
            'A' => CardJoker::Ace,
            'K' => CardJoker::King,
            'Q' => CardJoker::Queen,
            'T' => CardJoker::Ten,
            '9' => CardJoker::Nine,
            '8' => CardJoker::Eight,
            '7' => CardJoker::Seven,
            '6' => CardJoker::Six,
            '5' => CardJoker::Five,
            '4' => CardJoker::Four,
            '3' => CardJoker::Three,
            '2' => CardJoker::Two,
            'J' => CardJoker::Joker,
            _ => CardJoker::Two,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<Vec<Card>> for HandType {
    fn from(hand: Vec<Card>) -> Self {
        let counts = hand.iter().fold([0u8; 13], |mut counts, card| {
            counts[*card as usize] += 1;
            counts
        });
        let mut single = false;
        let mut quad = false;
        let mut triple = false;
        let mut double = false;
        let mut singles = 0;
        let mut pairs = 0;
        for count in counts {
            match count {
                5 => return Self::FiveOfAKind,
                4 => {
                    if single {
                        return Self::FourOfAKind;
                    }
                    quad = true;
                }
                3 => {
                    if double {
                        return Self::FullHouse;
                    }
                    if singles == 2 {
                        return Self::ThreeOfAKind;
                    }
                    triple = true;
                }
                2 => {
                    if triple {
                        return Self::FullHouse;
                    }
                    if double {
                        if single {
                            return Self::TwoPair;
                        }
                    }
                    if singles == 3 {
                        return Self::OnePair;
                    }
                    double = true;
                    pairs += 1;
                }
                1 => {
                    if quad {
                        return Self::FourOfAKind;
                    }
                    if triple {
                        if single {
                            return Self::ThreeOfAKind;
                        }
                    }
                    if pairs == 2 {
                        return Self::TwoPair;
                    }
                    if double {
                        if singles == 2 {
                            return Self::OnePair;
                        }
                    }
                    single = true;
                    singles += 1;
                }
                _ => (),
            }
        }
        Self::HighCard
    }
}

impl From<Vec<CardJoker>> for HandType {
    fn from(hand: Vec<CardJoker>) -> Self {
        let mut jokers = 0;
        let mut counts = hand.iter().fold([0u8; 12], |mut counts, card| {
            if card.is_joker() {
                jokers += 1;
            } else {
                counts[*card as usize] += 1;
            }
            counts
        });
        if jokers == 5 {
            return Self::FiveOfAKind;
        }
        counts.sort_unstable();
        counts.reverse();
        let mut triple = false;
        let mut singles = 0;
        let mut pairs = 0;
        for count in counts {
            match count {
                5 => return Self::FiveOfAKind,
                4 => {
                    if jokers == 1 {
                        return Self::FiveOfAKind;
                    }
                    return Self::FourOfAKind;
                }
                3 => {
                    if jokers == 2 {
                        return Self::FiveOfAKind;
                    }
                    if jokers == 1 {
                        return Self::FourOfAKind;
                    }
                    triple = true;
                }
                2 => {
                    if jokers == 3 {
                        return Self::FiveOfAKind;
                    }
                    if jokers == 2 {
                        return Self::FourOfAKind;
                    }
                    if jokers == 1 {
                        triple = true;
                        jokers = 0;
                    } else {
                        if triple {
                            return Self::FullHouse;
                        }
                        pairs += 1;
                    }
                }
                1 => {
                    if jokers == 4 {
                        return Self::FiveOfAKind;
                    }
                    if jokers == 3 {
                        return Self::FourOfAKind;
                    }
                    if jokers == 2 {
                        triple = true;
                        jokers = 0;
                    } else {
                        if jokers == 1 {
                            pairs += 1;
                            jokers = 0;
                        } else {
                            if triple && singles == 1 {
                                return Self::ThreeOfAKind;
                            }
                            if pairs == 2 {
                                return Self::TwoPair;
                            }
                            if pairs == 1 && singles == 2 {
                                return Self::OnePair;
                            }
                            singles += 1;
                        }
                    }
                }
                _ => (),
            }
        }
        Self::HighCard
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand<T> {
    typ: HandType,
    cards: [T; 5],
}

impl<T: Copy> From<Vec<T>> for Hand<T>
where
    Vec<T>: Into<HandType>,
{
    fn from(cards: Vec<T>) -> Self {
        Self {
            typ: cards.clone().into(),
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
        }
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let mut hands = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand: Hand<Card> = parts
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

    let score = hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u64 * bid)
        .sum::<u64>();
    println!("{score}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let mut hands = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand: Hand<CardJoker> = parts
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

    let score = hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u64 * bid)
        .sum::<u64>();
    println!("{score}");
}
