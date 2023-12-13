use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen, // 12
    King,  // 13
    Ace,   // 14
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
enum HandType {
    Highcard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    _cards: Vec<Card>,
    _kind: HandType,
    score: usize,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize, kind: HandType) -> Self {
        let score = Self::score(&kind, cards.as_slice());
        Hand {
            _cards: cards,
            _kind: kind,
            score,
            bid,
        }
    }

    fn score(kind: &HandType, cards: &[Card]) -> usize {
        let mut score = kind.clone() as usize;

        for c in cards {
            score = (score << 4) | *c as usize;
        }

        score
    }
}

fn get_card_type(cards: &Vec<Card>) -> Result<HandType, &str> {
    let mut card_counts = HashMap::new();

    for card in cards {
        let count = card_counts.entry(card).or_insert(0);
        *count += 1;
    }

    let mut hand_without_jokers = card_counts
        .keys()
        .filter_map(|x| {
            let values = card_counts[x];
            if x != &&Card::Joker {
                Some(values)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let jokers = card_counts.get(&Card::Joker).unwrap_or(&0);

    if jokers == &5 {
        return Ok(HandType::FiveOfAKind);
    }

    hand_without_jokers.sort_by(|a, b| b.cmp(a));

    if jokers > &0 && hand_without_jokers.len() >= 1 {
        hand_without_jokers[0] += jokers;
    }

    let mut final_hand = hand_without_jokers
        .iter()
        .filter(|x| **x >= 2)
        .collect::<Vec<_>>();

    final_hand.sort();

    match final_hand[..] {
        [2, 2] => return Ok(HandType::TwoPair),
        [2, 3] => return Ok(HandType::FullHouse),
        [2] => return Ok(HandType::Pair),
        [3] => return Ok(HandType::ThreeOfAKind),
        [4] => return Ok(HandType::FourOfAKind),
        [5] => return Ok(HandType::FiveOfAKind),
        _ => return Ok(HandType::Highcard),
    }
}

fn process(input: &str) -> Result<u32, &str> {
    let mut hands = input
        .lines()
        .into_iter()
        .map(|x| {
            let (hand, bid) = x.split_once(" ").unwrap();

            let cards = hand.chars().map(Card::from_char).collect::<Vec<_>>();
            let kind = get_card_type(&cards.to_vec()).unwrap();

            Hand::new(cards, bid.parse::<usize>().unwrap(), kind)
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.score.cmp(&b.score));

    let total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (x.bid * (i + 1)));

    Ok(total as u32)
}

fn main() {
    let input = include_str!("./input.txt");
    if let Ok(result) = process(input) {
        println!("Result: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn sample_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input).unwrap(), 5905);
    }

    #[test]
    fn highcard_upgrade() {
        let input = "AJ234".chars().map(Card::from_char).collect::<Vec<_>>();
        let kind = get_card_type(&input).unwrap();
        assert_eq!(kind, HandType::Pair);
    }

    #[test]
    fn joker_pair() {
        let input = "AJ234 4
AA234 8
";

        assert_eq!(process(input).unwrap(), 20);
    }

    #[test]
    fn regular_sorting() {
        let input = "JJJA2 4
KKKAA 8
";
        assert_eq!(process(input).unwrap(), 16);
    }

    #[rstest]
    #[case("23456", HandType::Highcard)]
    #[case("22KQA", HandType::Pair)]
    #[case("J2345", HandType::Pair)]
    #[case("KKQQ3", HandType::TwoPair)]
    #[case("22245", HandType::ThreeOfAKind)]
    #[case("J2245", HandType::ThreeOfAKind)]
    #[case("J2244", HandType::FullHouse)]
    #[case("22244", HandType::FullHouse)]
    #[case("JJJA2", HandType::FourOfAKind)]
    #[case("JJJAJ", HandType::FiveOfAKind)]
    #[case("JJJAA", HandType::FiveOfAKind)]
    #[case("JJJJJ", HandType::FiveOfAKind)]
    fn parsing_kind(#[case] input: &str, #[case] expected: HandType) {
        let transformed_input = input.chars().map(Card::from_char).collect::<Vec<_>>();
        assert_eq!(get_card_type(&transformed_input).unwrap(), expected);
    }

    #[rstest]
    #[case("AJJJ8", "AJJJ2", false)]
    #[case("AJJJ2", "AJJJ2", false)]
    #[case("QQQQ2", "JKKK2", false)]
    #[case("KKKK2", "QQQQ2", false)]
    #[case("2345J", "2346J", true)]
    #[case("AJJJ2", "AJJA2", true)]
    #[case("AJJJ2", "AJAA2", true)]
    #[case("AJJJ2", "AAAA2", true)]
    #[case("AJ452", "AA452", true)]
    #[case("JKKK2", "QQQQ2", true)]
    #[case("JKKK2", "QQQQ2", true)]
    fn calc_score(#[case] a: &str, #[case] b: &str, #[case] expected: bool) {
        let aa = a.chars().map(Card::from_char).collect::<Vec<_>>();
        let bb = b.chars().map(Card::from_char).collect::<Vec<_>>();

        let a_kind = get_card_type(&aa).unwrap();
        let b_kind = get_card_type(&bb).unwrap();

        let score_a = Hand::score(&a_kind, &aa);
        let score_b = Hand::score(&b_kind, &bb);

        assert_eq!(score_a < score_b, expected);
    }
}
