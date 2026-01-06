// this took 8 minutes to run but oh well

#[derive(Debug, Clone)]
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

fn main() {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let alpha = Deck::new(&alphabet);
    let beta = alpha.clone();

    // probably a better way to do this but oh well
    for c1 in 'A'..='Z' {
        println!("a"); // track progress
    for c2 in 'A'..='Z' {
    for c3 in 'A'..='Z' {
    for c4 in 'A'..='Z' {
    for c5 in 'A'..='Z' {
    for c6 in 'A'..='Z' {

        let msg = vec![c1, c2, c3, c4, c5, c6];

        if encrypt(&msg, alpha.clone(), beta.clone()) == vec!['L', 'P', 'W', 'G', 'Z', 'Z'] {
            println!("{}", msg.into_iter().collect::<String>());
        }

    }
    }
    }
    }
    }
    }
}
