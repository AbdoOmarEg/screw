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
//if player does something wrong it should be a result on what to do on the penalty

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let player1 = Player::new();
    let player2 = Player::new();
    let player3 = Player::new();
    let player4 = Player::new();

    let mut players = vec![player4, player1, player3, player2];

    let mut game = Game::new();

    println!("deck's len: {}", game.deck.len());
    for i in 0..4 {
        for j in 0..4 {
            players[i].cards[j] = game.deck.pop().unwrap();
        }
    }
    println!("deck's len: {}", game.deck.len());

    println!("printing players: ");
    for i in 0..4 {
        players[i].calculate_score();
        players[i].see_two_cards_on_the_right(i);
    }

    for i in 0..4 {
        println!("{:?}", players[i]);
        println!()
    }

    println!("{:?}", game.cards_on_ground);
    println!("deck's len: {}", game.deck.len());

    //players[0]'s turn
    //he has 5 options to swap from ground or match from ground or pull and discard (may activate special card)
    //or pull and swap from or pull and swap from one of your cards
    //let's try swap

    println!();
    println!("swapping");
    println!();

    let card_index = 0;
    players[0].swap(card_index, &mut game.cards_on_ground);
    game.make_new_card_on_ground_after_swap_visible();
    players[0].calculate_score();
    println!("{:?}", players[0]);
    println!("{:?}", game.cards_on_ground);

    //player's[0] 10, 1, +25, 6
    //3
    //lmao wanted to test calc score but the card on the ground was the same as the card was being
    //swapped
    //
    //now the swapping is good enough
    //just some cases if the deck is empty

    //player's[1]'s turn
    //he'll match
    println!();
    println!("matching");
    println!();

    let card = Card::Normal(NormalCard::ScrewDriver(0), vec![false; 4]);
    game.cards_on_ground.pop().unwrap();
    game.cards_on_ground.push(card.clone());
    println!("{:?}", game.cards_on_ground);

    players[1].cards[3] = card.clone();
    println!("{:?}", players[1]);

    players[1].match_the_card_on_the_ground(3, &mut game.cards_on_ground);

    //player's[2]'s turn
    //pull and discard

    println!();
    println!("pull and discard");
    println!();

    //player's[3]'s turn
    //pull and swap from the ground

    //doesn't exist
    println!();
    println!("pull and swap from ground");
    println!();

    //player's[0]'s turn
    //pull and swap from one of your hands

    println!();
    println!("pull and swap from one of your hands");
    println!();
}

// #[warn(dead_code)]
// #[warn(unused_variables)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn hello() {
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

    println!("{:?}", game.cards_on_ground);
    game.get_card_on_the_ground_from_deck();
    println!("{:?}", game.cards_on_ground);

    let card_index = 1;
    let card_on_ground = &mut game.cards_on_ground.last();
    // players[0].swap(card_index, &mut card_on_ground.last().unwrap());
    println!("{:?}", players[0]);
    game.make_new_card_on_ground_after_swap_visible();
    println!("{:?}", game.cards_on_ground);
    // game.card_on_ground = Card::Special(SpecialCard::Seven(10), vec![false; 4]);
    let top_card_on_deck = game.deck.last().unwrap();
    // it should be when player discard it
    match top_card_on_deck {
        Card::Normal(_, _) => (),
        Card::Special(special_cards, bol_vec) => match special_cards {
            SpecialCard::Seven(val) | SpecialCard::Eight(val) => {
                players[0].seven_eight(2);
            }
            // SpecialCard::Eight(val) => todo!(),
            SpecialCard::Nine(val) | SpecialCard::Ten(val) => {
                let mut other_player_cards = players[2].cards.clone();
                players[0].nine_ten(&mut other_player_cards, 2, 2, 0);
            }
            // SpecialCard::Ten(val) => todo!(),
            SpecialCard::Basra(val) => {
                let removed_card = players[0].basra(card_index);
                game.cards_on_ground.push(removed_card);
            }
            SpecialCard::Hatw5od(val) => {
                let mut other_player_cards = players[3].cards.clone();
                players[0].hatw5od(2, &mut other_player_cards, 3);
            }
            SpecialCard::Ka3bDayer(val) => {
                // players[0].ka3b_dayer(card_index);
                let card_index1 = 1;
                let card_index2 = 1;
                let card_index3 = 1;
                let card_index4 = 1;
                let mut player2_vec = players[1].cards.clone();
                let mut player3_vec = players[2].cards.clone();
                let mut player4_vec = players[3].cards.clone();
                players[0].ka3b_dayer(
                    card_index1,
                    card_index2,
                    card_index3,
                    card_index4,
                    &mut player2_vec,
                    &mut player3_vec,
                    &mut player4_vec,
                );
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

        // println!(
        //     "third card: {:?}, fourth card: {:?}",
        //     self.cards[2], self.cards[3]
        // );
    }

    fn calculate_score(&mut self) {
        let mut score = 0;
        for card in self.cards.iter() {
            score += card.calculate_value();
        }
        self.score = score;
    }

    fn swap(&mut self, card_index: usize, cards_on_ground: &mut Vec<Card>) {
        //swaps last card(top) on the ground with card of the player's choice
        std::mem::swap(
            &mut self.cards[card_index],
            &mut cards_on_ground.last_mut().unwrap(),
        )
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

    fn nine_ten(
        &mut self,
        other_player_cards: &mut Vec<Card>,
        card_other_player_index: usize,
        player_num: usize,
        the_player_looking: usize,
    ) {
        match &mut other_player_cards[card_other_player_index] {
            Card::Normal(normal_card, bol_vec) => {
                println!(
                    "you peeked {:?} at index {:?} from player {:?}",
                    normal_card, card_other_player_index, player_num
                );
                bol_vec[the_player_looking] = true;
            }
            Card::Special(normal_card, bol_vec) => {
                println!(
                    "you peeked {:?} at index {:?} from player {:?}",
                    normal_card, card_other_player_index, player_num
                );
                bol_vec[the_player_looking] = true;
            }
        }
        unimplemented!()
    }
    // was frustrating it not working but refcells will be a pain and this is a better approach,
    // and rust made me do the midly ok right choice
    // fn nine_ten(&mut self, other_player: &mut Player, card_other_player_index: usize) {
    //     //peek at other card player index and mark it as seen from you
    //     match &mut other_player.cards[card_other_player_index] {
    //         Card::Normal(normal_cards, bol_vec) => {
    //             bol_vec[card_other_player_index] = true;
    //             println!(
    //                 "you peeked {:?} at index {:?} from player {:?}",
    //                 other_player.cards[card_other_player_index],
    //                 card_other_player_index,
    //                 other_player
    //             );
    //         }
    //         Card::Special(normal_cards, bol_vec) => {
    //             bol_vec[card_other_player_index] = true;
    //             println!(
    //                 "you peeked {:?} at index {:?} from player {:?}",
    //                 other_player.cards[card_other_player_index],
    //                 card_other_player_index,
    //                 other_player
    //             );
    //         }
    //     };
    //     unimplemented!()
    // }
    //

    fn ka3b_dayer(
        &mut self,
        card_index1: usize,
        card_index2: usize,
        card_index3: usize,
        card_index4: usize,
        player2_vec: &mut Vec<Card>,
        player3_vec: &mut Vec<Card>,
        player4_vec: &mut Vec<Card>,
    ) {
        // change it to a loop and consider taking a vector instead of all the vector of cards of
        // each player
        println!(
            "player {:?} has card {:?} at {}",
            "One", self.cards[card_index1], card_index1
        );
        println!(
            "player {:?} has card {:?} at {}",
            "Two", player2_vec[card_index2], card_index2
        );
        println!(
            "player {:?} has card {:?} at {}",
            "Three", player3_vec[card_index3], card_index3
        );
        println!(
            "player {:?} has card {:?} at {}",
            "Four", player4_vec[card_index4], card_index4
        );

        match &mut self.cards[card_index1] {
            Card::Normal(_, bol) => {
                bol[0] = true;
            }
            Card::Special(_, bol) => {
                bol[0] = true;
            }
        }

        match &mut player2_vec[card_index2] {
            Card::Normal(_, bol) => {
                bol[0] = true;
            }
            Card::Special(_, bol) => {
                bol[0] = true;
            }
        }

        match &mut player3_vec[card_index3] {
            Card::Normal(_, bol) => {
                bol[0] = true;
            }
            Card::Special(_, bol) => {
                bol[0] = true;
            }
        }

        match &mut player4_vec[card_index4] {
            Card::Normal(_, bol) => {
                bol[0] = true;
            }
            Card::Special(_, bol) => {
                bol[0] = true;
            }
        }

        //adjust the seen boolean vec
        //make the index[0] (player1) true for all the cards here
        unimplemented!()
    }

    // fn ka3b_dayer(
    //     &self,
    //     card_index1: usize,
    //     card_index2: usize,
    //     card_index3: usize,
    //     card_index4: usize,
    //     players_vec: Vec<Player>,
    // ) {
    //     println!(
    //         "player {:?} has card {:?} at {}",
    //         players_vec[0], players_vec[0].cards[card_index1], card_index1
    //     );
    //     println!(
    //         "player {:?} has card {:?} at {}",
    //         players_vec[1], players_vec[1].cards[card_index2], card_index2
    //     );
    //     println!(
    //         "player {:?} has card {:?} at {}",
    //         players_vec[2], players_vec[2].cards[card_index3], card_index3
    //     );
    //     println!(
    //         "player {:?} has card {:?} at {}",
    //         players_vec[3], players_vec[3].cards[card_index4], card_index4
    //     );
    //     //adjust the seen boolean vec
    //     unimplemented!()
    // }

    fn basra(&mut self, card_index: usize) -> Card {
        self.cards.remove(card_index)
        // match x {
        //     Card::Normal(_, _) => todo!(),
        //     Card::Special(_, _) => todo!(),
        // }
    }

    fn hatw5od(
        &mut self,
        card_index: usize,
        other_player_cards: &mut Vec<Card>,
        other_card_index: usize,
    ) {
        std::mem::swap(
            &mut self.cards[card_index],
            &mut other_player_cards[other_card_index],
        );
        // no need to adjust the seen boolean vec
    }

    // fn hatw5od(
    //     &mut self,
    //     card_index: usize,
    //     other_player_cards: &mut Player,
    //     other_card_index: usize,
    // ) {
    //     // fn swap(&mut self, card_index: usize, card_on_ground: &mut Card) {
    //     //     let temp1 = &mut self.cards[card_index];
    //     //     println!("DEBUGPRINT[2]: main.rs:145: temp1={:#?}", temp1);
    //     //     let temp2 = card_on_ground;
    //     //     std::mem::swap(temp1, temp2)
    //     // }
    //     let temp1 = &mut self.cards[card_index];
    //     let temp2 = &mut other_player.cards[other_card_index];
    //     std::mem::swap(temp1, temp2);
    //     unimplemented!()
    // }

    fn match_the_card_on_the_ground(&mut self, card_index: usize, cards_on_ground: &mut Vec<Card>) {
        //should return result
        if &mut self.cards[card_index] == cards_on_ground.last_mut().unwrap() {
            std::mem::swap(
                &mut self.cards[card_index],
                cards_on_ground.last_mut().unwrap(),
            );
            println!("did match");
        } else {
            self.cards.push(cards_on_ground.last_mut().unwrap().clone());
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
    cards_on_ground: Vec<Card>,
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
            cards_on_ground: card_on_ground,
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
        self.cards_on_ground.push(card_on_ground);
    }

    fn round_counter(&mut self) {
        self.rounds += 1;
    }

    fn make_new_card_on_ground_after_swap_visible(&mut self) {
        match &mut self.cards_on_ground.last_mut().unwrap() {
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
