advent_of_code::solution!(7);

use std::collections::HashMap;

enum Part {
    One,
    Two,
}

#[derive(Debug, Eq, Default, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum CardValue {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

#[derive(Debug, Eq, Default, PartialEq, Clone, Copy)]
struct Card {
    value: CardValue,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (card1, card2) in self.cards.iter().zip(&other.cards) {
            let card_ordering = card1.cmp(card2);
            if card_ordering != std::cmp::Ordering::Equal {
                return card_ordering;
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, Ord, Clone, Copy, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct CamelCardsPlay {
    hand: Hand,
    bid: u32,
    level: HandType,
}

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn parse_card_level(c: char, part: Part) -> CardValue {
    match c {
        '2' => CardValue::Two,
        '3' => CardValue::Three,
        '4' => CardValue::Four,
        '5' => CardValue::Five,
        '6' => CardValue::Six,
        '7' => CardValue::Seven,
        '8' => CardValue::Eight,
        '9' => CardValue::Nine,
        'T' => CardValue::Ten,
        'J' => match part {
            Part::One => CardValue::Jack,
            Part::Two => CardValue::One,
        },
        'Q' => CardValue::Queen,
        'K' => CardValue::King,
        'A' => CardValue::Ace,
        _ => CardValue::Joker,
    }
}

fn determine_card_groups(hand: &str) -> (u32, u32) {
    let mut card_counts: HashMap<char, u32> = HashMap::new();

    for card in hand.chars() {
        let counter = card_counts.entry(card).or_insert(0);
        *counter += 1;
    }

    // build out vector of card counts sorted in descending order
    let mut sorted_card_counts: Vec<u32> = card_counts.values().cloned().collect();
    sorted_card_counts.sort_by(|a, b| b.cmp(a));

    // only return top 2 card groups
    if sorted_card_counts.len() >= 2 {
        (sorted_card_counts[0], sorted_card_counts[1])
    } else {
        (sorted_card_counts[0], 0)
    }
}

fn determine_hand_type(hand: &str) -> HandType {
    let level: HandType;

    let first_card = hand.chars().next().unwrap();
    let (first_group, second_group) = determine_card_groups(hand);

    if hand.chars().all(|card| card == first_card) {
        level = HandType::FiveOfAKind;
    } else if first_group == 4 {
        level = HandType::FourOfAKind;
    } else if first_group == 3 && second_group == 2 {
        level = HandType::FullHouse;
    } else if first_group == 3 {
        level = HandType::ThreeOfAKind;
    } else if first_group == 2 && second_group == 2 {
        level = HandType::TwoPair;
    } else if first_group == 2 {
        level = HandType::OnePair;
    } else {
        level = HandType::HighCard;
    }

    level
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_winnings: u32 = 0;
    let lines = prepare_input(input);

    let mut camel_cards: Vec<CamelCardsPlay> = vec![];

    for line in lines {
        // parse each line in the input for a hand and a bid
        let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
        let hand: String = split_line[0].to_string();
        let bid: u32 = split_line[1].parse().unwrap();

        // determine level/rank of each hand
        let level = determine_hand_type(&hand);

        // build out hands from cards and card values to leverage sorting
        let mut typed_hand: [Card; 5] = Default::default();
        for (index, char) in hand.chars().enumerate() {
            let card_value = parse_card_level(char, Part::One);
            let card = Card { value: card_value };
            typed_hand[index] = card;
        }
        camel_cards.push(CamelCardsPlay {
            hand: Hand { cards: typed_hand },
            bid,
            level,
        });
    }

    // sort hands
    camel_cards.sort_by_key(|d| d.hand);
    camel_cards.sort_by_key(|d| d.level);

    // multiply bid by rank for each and calculate total
    for (index, play) in camel_cards.iter().enumerate() {
        total_winnings += (index as u32 + 1) * play.bid;
    }

    Some(total_winnings)
}

fn determine_card_groups_jokers(hand: &str) -> (u32, u32) {
    let mut card_counts: HashMap<char, u32> = HashMap::new();
    let mut joker_counter: u32 = 0;

    // determine card groups like before, but hold off on jokers ("J")
    for card in hand.chars() {
        let counter = card_counts.entry(card).or_insert(0);
        if card != 'J' {
            *counter += 1;
        } else {
            joker_counter += 1;
        }
    }

    let mut sorted_card_counts: Vec<u32> = card_counts.values().cloned().collect();
    sorted_card_counts.sort_by(|a, b| b.cmp(a));

    // add all jokers to card group with the highest count
    sorted_card_counts[0] += joker_counter;

    if sorted_card_counts.len() >= 2 {
        (sorted_card_counts[0], sorted_card_counts[1])
    } else {
        (sorted_card_counts[0], 0)
    }
}

fn determine_hand_type_jokers(hand: &str) -> HandType {
    let level: HandType;

    let (first_group, second_group) = determine_card_groups_jokers(hand);

    if first_group == 5 {
        level = HandType::FiveOfAKind;
    } else if first_group == 4 {
        level = HandType::FourOfAKind;
    } else if first_group == 3 && second_group == 2 {
        level = HandType::FullHouse;
    } else if first_group == 3 {
        level = HandType::ThreeOfAKind;
    } else if first_group == 2 && second_group == 2 {
        level = HandType::TwoPair;
    } else if first_group == 2 {
        level = HandType::OnePair;
    } else {
        level = HandType::HighCard;
    }

    level
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_winnings: u32 = 0;
    let lines = prepare_input(input);

    let mut camel_cards: Vec<CamelCardsPlay> = vec![];

    for line in lines {
        // parse each line in the input for a hand and a bid
        let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
        let hand: String = split_line[0].to_string();
        let bid: u32 = split_line[1].parse().unwrap();

        // determine level/rank of each hand
        let level = determine_hand_type_jokers(&hand);

        // build out hands from cards and card values to leverage sorting
        let mut typed_hand: [Card; 5] = Default::default();
        for (index, char) in hand.chars().enumerate() {
            let card_value = parse_card_level(char, Part::Two);
            let card = Card { value: card_value };
            typed_hand[index] = card;
        }
        camel_cards.push(CamelCardsPlay {
            hand: Hand { cards: typed_hand },
            bid,
            level,
        });
    }

    // sort hands
    camel_cards.sort_by_key(|d| d.hand);
    camel_cards.sort_by_key(|d| d.level);

    // multiply bid by rank for each and calculate total
    for (index, play) in camel_cards.iter().enumerate() {
        total_winnings += (index as u32 + 1) * play.bid;
    }

    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 6440);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 5905);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(determine_hand_type("32T3K"), HandType::OnePair,);
        assert_eq!(determine_hand_type("T55J5"), HandType::ThreeOfAKind,);
        assert_eq!(determine_hand_type("KK677"), HandType::TwoPair,);
        assert_eq!(determine_hand_type("KTJJT"), HandType::TwoPair,);
        assert_eq!(determine_hand_type("QQQJA"), HandType::ThreeOfAKind,);

        assert_eq!(determine_hand_type_jokers("32T3K"), HandType::OnePair,);
        assert_eq!(determine_hand_type_jokers("T55J5"), HandType::FourOfAKind,);
        assert_eq!(determine_hand_type_jokers("KK677"), HandType::TwoPair,);
        assert_eq!(determine_hand_type_jokers("KKJ77"), HandType::FullHouse,);
        assert_eq!(determine_hand_type_jokers("JKK77"), HandType::FullHouse,);
        assert_eq!(determine_hand_type_jokers("KTJJT"), HandType::FourOfAKind,);
        assert_eq!(determine_hand_type_jokers("QQQJA"), HandType::FourOfAKind,);
        assert_eq!(determine_hand_type_jokers("J2345"), HandType::OnePair,);
        assert_eq!(determine_hand_type_jokers("2J345"), HandType::OnePair,);
        assert_eq!(determine_hand_type_jokers("23J45"), HandType::OnePair,);
        assert_eq!(determine_hand_type_jokers("234J5"), HandType::OnePair,);
        assert_eq!(determine_hand_type_jokers("2345J"), HandType::OnePair,);

        assert_eq!(determine_hand_type("JJJJJ"), HandType::FiveOfAKind);
        assert_eq!(determine_hand_type("JJJJJ"), HandType::FiveOfAKind);
        assert_eq!(determine_hand_type("JJJJQ"), HandType::FourOfAKind);
        assert_eq!(determine_hand_type_jokers("JJJJQ"), HandType::FiveOfAKind);
        assert_eq!(determine_hand_type("JJQJJ"), HandType::FourOfAKind);
        assert_eq!(determine_hand_type_jokers("JJQJJ"), HandType::FiveOfAKind);
    }
}
