

fn main() {
    let p = Player{name: String::from("Rosalyn")};
    p.perform(&Action::Shout);
    p.perform(&Action::Attack(String::from("Derek")));
    p.perform(&Action::Move{x: 3, y: -190});
    p.perform(&Action::Matchmake(
        PlayerCouple{
            one: String::from("Whitney"),
            two: String::from("Ashley"),
        }
    ));

}

struct Player {
    name: String
}

impl Player {
    fn perform(&self, a: &Action) {
        match a {
            Action::Shout => println!("AAAH!"),
            Action::Attack(who) => println!("I am hitting you, {}!", who),
            Action::Move{y, x} => println!("I moving to coordinates: ({}, {})", x, y),
            Action::Matchmake(PlayerCouple{one: a, two: b}) => println!("Hey, {}, what do you think about {}?", a, b),
        }
    }
}

enum Action {
    Shout,
    //Talk(String),
    Attack(String),
    Move{x: i16, y: i16},
    Matchmake(PlayerCouple),
}

struct PlayerCouple {
    one: String,
    two: String,
}


