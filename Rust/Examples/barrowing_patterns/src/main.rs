/* Borrowing Patterns and examples
  - New Scopes
  - Temporary variables
  - Entry API
  - Splitting up structs
*/

use std::collections::HashMap;

fn main_new_scopes() {
    let mut list = vec![1, 2, 3];

    // Create inner scope within main
    // immutable borrows happen in this scope
    {
        let list_first = list.first();
        let list_last = list.last();

        println!(
            "The first element is {:?} and the last element is {:?}",
            list_first,
            list_last,
        );
    }

    // mutable borrows happen here so compiler thinks the borrows don't happen at the same time
    *list.first_mut().expect("list was empty") += 1;
}

// Temporary variable to hold result
// Causes borrow to end when result is computed
pub struct Player {
    score: i32,
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

fn main_for_temp_variable() {
    let mut player1 = Player::new();
    // Introduce temporary variable here
    let old_score = player1.score();
    player1.set_score(old_score + 1);
}

// Example using a hashmap to count word frequencies
fn main_hashmap() {
    let text = "hello my friend hello";

    let mut freqs = HashMap::new();
    for word in text.split_whitespace() {
        match freqs.get_mut(word) {
            Some(value) => *value += 1,
            None => {
                freqs.insert(word, 1);
            }
        }
    }

    println!("Word frequencies: {:#?}", freqs);
}

// Another example using hashmap
// or_insert method
// pub fn or_insert(self, default: V) -> &'a mut V

fn main_for_hashmap_using_or_insert() {
    let text = "hello my friend hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);
}

// Splitting up structs into multiple structs
pub struct Stats {
    hp: u8,
    sp: u8,
}

pub struct Monster {
    stats: Stats,
    friends: Vec<Friend>,
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_breath(&mut self) {
        if let Some(friend) = self.friends.first() {
            self.stats.heal(friend.loyalty);
            println!("Healing for {}", friend.loyalty);
        }
    }
}

impl Stats {
    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

fn main () {
    //let godzilla = Monster::new();
    //godzilla.final_breath();
    //let mut m = Monster::new();
}

// Upcoming:
// Non-Lexical Lifetimes (NLL)
// Borrow checkers don't always have to wait until the end of executable scope
// Entry API is still preferred