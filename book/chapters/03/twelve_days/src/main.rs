
fn main() {
    let lyrics: Vec<&str> = LYRICS.split("\n").collect();
    let ordinals: Vec<&str> = ORDINALS.split("\n").collect();
    println!("First lyric: {}", lyrics[lyrics.len() - 1]);
    println!("Num ordinals, lyrics: {}, {}", ordinals.len(), lyrics.len());
    if ordinals.len() != lyrics.len() {
        panic!("Need a lyric for each ordinal.");
    }
    for i in 0..lyrics.len() {
        println!("On the {} day of Christmas, my true love sent to me", ordinals[i]);
        for j in 0..=i {
            println!("{}", lyrics[(11 - i) + j]);
        }
        println!("");
    }
}

const LYRICS: &str= "Twelve drummers drumming
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree";

const ORDINALS: &str = "first
second
third
fourth
fifth
sixth
seventh
eighth
ninth
tenth
eleventh
twelfth";


