use std::io;

#[derive(Debug)]
struct Deck {
    cards: Vec<char>,
}

impl Deck {
    fn new(cards: &[char]) -> Self {
        Self { cards: cards.to_vec() }
    }

    fn split(&self) -> [Self; 2] {
        [
            Self::new(&self.cards[..self.cards.len()/2]),
            Self::new(&self.cards[self.cards.len()/2..])
        ]
    }

    fn combine(left: Self, right: Self) -> Self {
        let mut cards = Vec::new();
        for i in 0..left.cards.len() {
            cards.push(right.cards[i]);
            cards.push(left.cards[i]);
        }

        Self { cards }
    }

    fn rotate(&mut self) {
        self.cards.push(self.cards[0]);
        self.cards.remove(0);
    }

    fn to_alpha_beta(&self) -> [Self; 2] {
        let mut alpha = Vec::new();
        let mut beta = Vec::new();

        for card in &self.cards {
            if alpha.contains(card) {
                beta.push(*card);
            } else {
                alpha.push(*card);
            }
        }

        [Self { cards: alpha }, Self { cards: beta }]
    }

    fn send_to_middle(&mut self, i: usize) {
        let middle = self.cards.len().div_ceil(2);

        let card = self.cards[i];
        self.cards.remove(i);
        self.cards.insert(middle, card);
    }
}

fn generate_decks(n: u32, s: u32, c: u32) -> [Deck; 2] {
    let mut cards = Vec::new();
    (0..n).map(|i| char::from_u32(i + 'A' as u32).unwrap())
        .for_each(|card| for _ in 0..2 { cards.push(card) });

    let mut combined_deck = Deck::new(&cards);

    for _ in 0..s {
        let [left, right] = combined_deck.split();

        combined_deck = Deck::combine(left, right);

        for _ in 0..c {
            combined_deck.rotate();
        }
    }

    combined_deck.to_alpha_beta()
}

fn encrypt(msg: &[char], mut alpha: Deck, mut beta: Deck) -> Vec<char> {
    let mut cyphertext = Vec::new();

    for c in msg {
        while alpha.cards[0] != *c {
            alpha.rotate();
            beta.rotate();
        }

        cyphertext.push(beta.cards[0]);

        alpha.send_to_middle(1);

        beta.rotate();
        beta.send_to_middle(2);
    }

    cyphertext
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut tokens = input.split_whitespace();

    let n: u32 = tokens.next().unwrap().parse().unwrap();
    let s: u32 = tokens.next().unwrap().parse().unwrap();
    let c: u32 = tokens.next().unwrap().parse().unwrap();

    let msg: Vec<char> = tokens.next().unwrap().chars().collect();

    let [alpha, beta] = generate_decks(n, s, c);
    // println!("{:?}\n{:?}", alpha, beta);

    println!("{}", encrypt(&msg, alpha, beta)
        .into_iter()
        .collect::<String>());

    Ok(())
}

