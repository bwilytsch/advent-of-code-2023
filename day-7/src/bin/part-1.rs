#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,  // 11
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
            'J' => Card::Jack,
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
    score: usize,
    bid: usize,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: usize) -> Self {
        let score = Self::score(&cards);
        Hand {
            _cards: cards,
            score,
            bid,
        }
    }

    fn score(cards: &[Card]) -> usize {
        let kind = get_card_type(&cards.to_vec()).unwrap();

        let mut score = kind as usize;

        for c in cards {
            score = (score << 4) | *c as usize;
        }

        score
    }
}

fn get_card_type(cards: &Vec<Card>) -> Result<HandType, &str> {
    let mut card_counts = std::collections::HashMap::new();

    for card in cards {
        let count = card_counts.entry(card).or_insert(0);
        *count += 1;
    }

    let mut result = card_counts
        .values()
        .filter(|x| **x >= 2)
        .collect::<Vec<_>>();

    // Only needed for FullHouse
    result.sort();

    match result[..] {
        [2, 2] => return Ok(HandType::TwoPair),
        [2, 3] => return Ok(HandType::FullHouse),
        [2] => return Ok(HandType::Pair),
        [3] => return Ok(HandType::ThreeOfAKind),
        [4] => return Ok(HandType::FourOfAKind),
        [5] => return Ok(HandType::FiveOfAKind),
        _ => return Ok(HandType::Highcard),
    }
}

// Calculate total winnings from all hands & bids
fn process(input: &str) -> Result<u32, &str> {
    let mut hands = input
        .lines()
        .into_iter()
        .map(|x| {
            let (hand, bid) = x.split_once(" ").unwrap();

            let cards = hand.chars().map(Card::from_char).collect::<Vec<_>>();

            Hand::new(cards, bid.parse::<usize>().unwrap())
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

    #[test]
    fn basic() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input).unwrap(), 6440);
    }

    #[test]
    fn sorting() {
        let input = include_str!("./input.txt");

        let result_one = process(input).unwrap();
        let result_two = process(input).unwrap();

        assert_eq!(result_one, result_two);
    }
}
