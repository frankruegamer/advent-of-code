use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    pub bid: usize,
}

#[derive(Debug)]
pub struct ParseCardsError;

impl FromStr for Hand {
    type Err = ParseCardsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(ParseCardsError)?;

        let cards = cards
            .chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<_, _>>()
            .map_err(|_| ParseCardsError)?;

        let bid = bid.parse().map_err(|_| ParseCardsError)?;

        Ok(Hand { cards, bid })
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // (self.rank(), &self.cards[0], &self.cards[1], &self.cards[2], &self.cards[3], &self.cards[4]).cmp(&(other.rank(), &other.cards[0], &other.cards[1], &other.cards[]))
        (self.hand_type(), &self.cards).cmp(&(other.hand_type(), &other.cards))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut map = self
            .cards
            .iter()
            .fold(HashMap::<&Card, u8>::new(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            });

        if !Self::remove_any_with(&mut map, 5).is_empty() {
            return HandType::FiveOfAKind;
        }
        if !Self::remove_any_with(&mut map, 4).is_empty() {
            return HandType::FourOfAKind;
        }
        if !Self::remove_any_with(&mut map, 3).is_empty() {
            return if !Self::remove_any_with(&mut map, 2).is_empty() {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            };
        }
        match Self::remove_any_with(&mut map, 2).len() {
            2 => return HandType::TwoPair,
            1 => return HandType::OnePair,
            _ => {}
        }
        HandType::HighCard
    }

    fn remove_any_with<'a>(map: &mut HashMap<&'a Card, u8>, count: u8) -> Vec<(&'a Card, u8)> {
        let a: Vec<_> = map
            .iter()
            .filter(|(_, num)| **num == count)
            .map(|(card, _)| card)
            .copied()
            .collect();
        let mut vec = Vec::new();
        for card in a {
            vec.push(map.remove_entry(card).unwrap());
        }
        vec
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    J,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = ParseCardsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card = match s {
            "2" => Card::Two,
            "3" => Card::Three,
            "4" => Card::Four,
            "5" => Card::Five,
            "6" => Card::Six,
            "7" => Card::Seven,
            "8" => Card::Eight,
            "9" => Card::Nine,
            "T" => Card::Ten,
            "J" => Card::J,
            "Q" => Card::Q,
            "K" => Card::K,
            "A" => Card::A,
            _ => Err(ParseCardsError)?,
        };
        Ok(card)
    }
}
