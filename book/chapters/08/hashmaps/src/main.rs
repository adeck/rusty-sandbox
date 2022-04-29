
use std::collections::HashMap;

fn main() {
    simple_hashin();
    zippin_tuples();
    iteratin();
}

fn iteratin() {
    let line = "we like things like that";
    let mut word_count = HashMap::new();
    for word in line.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_count);
}

fn zippin_tuples() {
    let teams = vec!["Red".to_string(), "Blue".to_string()];
    let scores = vec![10, 25];
    let team_scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    println!("Team scores: {:?}", &team_scores);
}

fn simple_hashin() {
    // Here we're mapping menu items to prices in dollars
    let mut map = HashMap::new();
    map.insert("Eggs on Toast".to_string(), 6);
    map.insert("Bowl of Cereal".to_string(), 4);
    map.insert("Broken Glass".to_string(), 2);
    map.insert("Alec Baldwin".to_string(), 3);
    println!("Menu prices: {:?}", &map);

}
