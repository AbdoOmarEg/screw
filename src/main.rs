use rand::seq::SliceRandom;
use rand::thread_rng;

//TODO: card on the ground: DONE
//popping a card from the deck: DONE
//swap a card of yours from the ground: DONE
//if I want to have a fn that have the players and the game fn,
//I will have to do refcells(the devil)
//7, 8 peek a card
//9, 10 peek a card from other player
//Ka3bDayer peek a card from all players including you
//Hatw5od swap without peekin
//basra discard a card of your choice into the ground
//and I think that's it
//get card from deck and put it on the ground
//after every fn you should calculate_score for all players
fn main() {
    let player1 = Player::new();
    let player2 = Player::new();
    let player3 = Player::new();
    let player4 = Player::new();

    let mut players = vec![player4, player1, player3, player2];

    let mut game = Game::new();

    for i in 0..4 {
        for j in 0..4 {
            players[i].cards[j] = game.deck.pop().unwrap();
        }
    }

    println!("printing players: ");
    for i in 0..4 {
        players[i].calculate_score();
        players[i].see_two_cards_on_the_right(i);
    }

    for i in 0..4 {
        println!("{:?}", players[i]);
    }

    println!("{:?}", game.card_on_ground);
    game.get_card_on_the_ground_from_deck();
    println!("{:?}", game.card_on_ground);

    let card_index = 1;
    let card_on_ground = &mut game.card_on_ground.last();
    // players[0].swap(card_index, &mut card_on_ground.last().unwrap());
    println!("{:?}", players[0]);
    game.make_new_card_on_ground_after_swap_visible();
    println!("{:?}", game.card_on_ground);
    // game.card_on_ground = Card::Special(SpecialCard::Seven(10), vec![false; 4]);
    let top_card_on_deck = game.deck.last().unwrap();
    match top_card_on_deck {
        Card::Normal(_, _) => (),
        Card::Special(special_cards, bol_vec) => match special_cards {
            SpecialCard::Seven(val) | SpecialCard::Eight(val) => {
                players[0].seven_eight(2);
            }
            // SpecialCard::Eight(val) => todo!(),
            SpecialCard::Nine(val) | SpecialCard::Ten(val) => {
                // players[0].nine_ten(&mut players[2], 2);
            }
            // SpecialCard::Ten(val) => todo!(),
            SpecialCard::Basra(val) => {
                // players[0].basra(card_index);
            }
            SpecialCard::Hatw5od(val) => {
                // players[0].hatw5od(card_index);
            }
            SpecialCard::Ka3bDayer(val) => {
                // players[0].ka3b_dayer(card_index);
            }
        },
    }
}

#[derive(Debug)]
struct Player {
    cards: Vec<Card>,
    // should be put on the game struct
    // can_screw: bool,
    score: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum Card {
    Normal(NormalCard, Vec<bool>),
    Special(SpecialCard, Vec<bool>),
}

impl Card {
    fn calculate_value(&self) -> i32 {
        match self {
            Card::Normal(normal_card, _) => match normal_card {
                NormalCard::One(value) => *value,
                NormalCard::Two(value) => *value,
                NormalCard::Three(value) => *value,
                NormalCard::Four(value) => *value,
                NormalCard::Five(value) => *value,
                NormalCard::Six(value) => *value,
                NormalCard::MinusOne(value) => *value,
                NormalCard::ScrewDriver(value) => *value,
                NormalCard::Plus20(value) => *value,
                NormalCard::RedScrew(value) => *value,
            },
            Card::Special(special_card, _) => match special_card {
                SpecialCard::Seven(value) => *value,
                SpecialCard::Eight(value) => *value,
                SpecialCard::Nine(value) => *value,
                SpecialCard::Ten(value) => *value,
                SpecialCard::Basra(value) => *value,
                SpecialCard::Hatw5od(value) => *value,
                SpecialCard::Ka3bDayer(value) => *value,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NormalCard {
    One(i32),
    Two(i32),
    Three(i32),
    Four(i32),
    Five(i32),
    Six(i32),
    MinusOne(i32),
    ScrewDriver(i32),
    Plus20(i32),
    RedScrew(i32),
}

#[derive(Debug, Clone, PartialEq)]
enum SpecialCard {
    Seven(i32),
    Eight(i32),
    Nine(i32),
    Ten(i32),
    Basra(i32),
    Hatw5od(i32),
    Ka3bDayer(i32),
}

impl Player {
    fn new() -> Self {
        let cards = vec![Card::Normal(NormalCard::One(1), vec![false; 4]); 4];
        let score = 0;
        Self { cards, score }
    }

    fn see_two_cards_on_the_right(&mut self, i: usize) {
        match &mut self.cards[2] {
            Card::Normal(_, bolbol) => bolbol[i] = true,
            Card::Special(_, bolbol) => bolbol[i] = true,
        };

        match &mut self.cards[3] {
            Card::Normal(_, bolbol) => bolbol[i] = true,
            Card::Special(_, bolbol) => bolbol[i] = true,
        };

        // (self.seen_cards[2], self.seen_cards[3]) = (true, true);

        println!(
            "third card: {:?}, fourth card: {:?}",
            self.cards[2], self.cards[3]
        );
    }

    fn calculate_score(&mut self) {
        let mut score = 0;
        for card in self.cards.iter() {
            score += card.calculate_value();
        }
        self.score = score;
    }

    fn swap(&mut self, card_index: usize, card_on_ground: &mut Card) {
        let temp1 = &mut self.cards[card_index];
        println!("DEBUGPRINT[2]: main.rs:145: temp1={:#?}", temp1);
        let temp2 = card_on_ground;
        std::mem::swap(temp1, temp2)
    }

    fn seven_eight(&mut self, card_index: usize) {
        //make it be seen in the vec
        match &mut self.cards[card_index] {
            Card::Normal(val, bol) => {
                println!("the value of {card_index} is: {:?}", val);
                bol[card_index] = true;
            }
            Card::Special(val, bol) => {
                println!("the value of {card_index} is: {:?}", val);
                bol[card_index] = true;
            }
        }
    }

    fn nine_ten(&mut self, other_player: &mut Player, card_other_player_index: usize) {
        //peek at other card player index and mark it as seen from you
        match &mut other_player.cards[card_other_player_index] {
            Card::Normal(normal_cards, bol_vec) => {
                bol_vec[card_other_player_index] = true;
                println!(
                    "you peeked {:?} at index {:?} from player {:?}",
                    other_player.cards[card_other_player_index],
                    card_other_player_index,
                    other_player
                );
            }
            Card::Special(normal_cards, bol_vec) => {
                bol_vec[card_other_player_index] = true;
                println!(
                    "you peeked {:?} at index {:?} from player {:?}",
                    other_player.cards[card_other_player_index],
                    card_other_player_index,
                    other_player
                );
            }
        };
        unimplemented!()
    }

    fn ka3b_dayer(
        &self,
        card_index1: usize,
        card_index2: usize,
        card_index3: usize,
        card_index4: usize,
        players_vec: Vec<Player>,
    ) {
        println!(
            "player {:?} has card {:?} at {}",
            players_vec[0], players_vec[0].cards[card_index1], card_index1
        );
        println!(
            "player {:?} has card {:?} at {}",
            players_vec[1], players_vec[1].cards[card_index2], card_index2
        );
        println!(
            "player {:?} has card {:?} at {}",
            players_vec[2], players_vec[2].cards[card_index3], card_index3
        );
        println!(
            "player {:?} has card {:?} at {}",
            players_vec[3], players_vec[3].cards[card_index4], card_index4
        );
        unimplemented!()
    }

    fn basra(&mut self, card_index: usize) {
        self.cards.remove(card_index);
        // match x {
        //     Card::Normal(_, _) => todo!(),
        //     Card::Special(_, _) => todo!(),
        // }

        unimplemented!()
    }

    fn hatw5od(&mut self, card_index: usize, other_player: &mut Player, other_card_index: usize) {
        // fn swap(&mut self, card_index: usize, card_on_ground: &mut Card) {
        //     let temp1 = &mut self.cards[card_index];
        //     println!("DEBUGPRINT[2]: main.rs:145: temp1={:#?}", temp1);
        //     let temp2 = card_on_ground;
        //     std::mem::swap(temp1, temp2)
        // }
        let temp1 = &mut self.cards[card_index];
        let temp2 = &mut other_player.cards[other_card_index];
        std::mem::swap(temp1, temp2);
        unimplemented!()
    }

    fn match_the_card_on_the_ground(&mut self, card_index: usize, card_on_ground: &mut Card) {
        if &mut self.cards[card_index] == card_on_ground {
            std::mem::swap(&mut self.cards[card_index], card_on_ground)
        } else {
            self.cards.push(card_on_ground.clone());
            //reveal the previous card on ground
            //so card on ground must be a vector then
        }
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    can_screw: bool,
    deck: Vec<Card>,
    rounds: i32,
    card_on_ground: Vec<Card>,
}

impl Game {
    fn new() -> Self {
        let mut deck = Vec::with_capacity(57);
        for _ in 0..4 {
            deck.push(Card::Normal(NormalCard::One(1), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::Two(2), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::Three(3), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::Four(4), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::Five(5), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::Six(6), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::Plus20(20), vec![false; 4]));
            deck.push(Card::Special(SpecialCard::Seven(10), vec![false; 4]));
            deck.push(Card::Special(SpecialCard::Eight(10), vec![false; 4]));
            deck.push(Card::Special(SpecialCard::Nine(10), vec![false; 4]));
            deck.push(Card::Special(SpecialCard::Ten(10), vec![false; 4]));
            deck.push(Card::Special(SpecialCard::Hatw5od(10), vec![false; 4]));
        }

        for _ in 0..2 {
            deck.push(Card::Special(SpecialCard::Basra(10), vec![false; 4]));
            deck.push(Card::Special(SpecialCard::Ka3bDayer(10), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::RedScrew(25), vec![false; 4]));
            deck.push(Card::Normal(NormalCard::ScrewDriver(0), vec![false; 4]));
            // can make a nested 0..2 loop for the the 4 cards inside here
        }

        for _ in 0..1 {
            deck.push(Card::Normal(NormalCard::MinusOne(-1), vec![false; 4]));
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        let players = Vec::with_capacity(4);

        //will be popping of the deck
        //card_on_ground will be visible to anyone as it is on the ground
        let mut card_on_ground = deck.pop().unwrap();
        match &mut card_on_ground {
            Card::Normal(_, bolbol) => {
                for i in 0..bolbol.len() {
                    bolbol[i] = true;
                }
            }
            Card::Special(_, bolbol) => {
                for i in 0..bolbol.len() {
                    bolbol[i] = true;
                }
            }
        };
        let card_on_ground = vec![card_on_ground];
        // let card_on_ground = self::Game::get_card_on_the_ground_from_deck(&mut Game);

        Self {
            players,
            can_screw: false,
            deck,
            rounds: 0,
            card_on_ground,
        }
    }

    fn get_card_on_the_ground_from_deck(&mut self) {
        println!("{:?}", self.deck.len());
        let mut card_on_ground = self.deck.pop().unwrap();
        match &mut card_on_ground {
            Card::Normal(_, bolbol) => {
                for i in 0..bolbol.len() {
                    bolbol[i] = true;
                }
            }
            Card::Special(_, bolbol) => {
                for i in 0..bolbol.len() {
                    bolbol[i] = true;
                }
            }
        };
        println!("{:?}", self.deck.len());
        self.card_on_ground.push(card_on_ground);
    }

    fn round_counter(&mut self) {
        self.rounds += 1;
    }

    fn make_new_card_on_ground_after_swap_visible(&mut self) {
        match &mut self.card_on_ground.last_mut().unwrap() {
            Card::Normal(_, bolbol) => {
                for i in 0..bolbol.len() {
                    bolbol[i] = true;
                }
            }
            Card::Special(_, bolbol) => {
                for i in 0..bolbol.len() {
                    bolbol[i] = true;
                }
            }
        }
    }

    // fn swap(&mut self, player_index: usize, card_index: usize) {
    //     let x = &mut self.players[player_index].cards[card_index];
    //     println!("DEBUGPRINT[1]: main.rs:236: x={:#?}", x);
    // }
}
