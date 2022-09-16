use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
struct Wizard {
    #[eg = "Harry Potter"]
    name: String,
    #[eg = "11"]
    age: usize,
    #[eg = "totally_random_spell"]
    fav_spell: Spell,
}

#[derive(Debug, PartialEq)]
enum Spell {
    Expelliarmus,
    #[allow(dead_code)]
    WinguardiumLeviosa,
}

fn totally_random_spell() -> Spell {
    Spell::Expelliarmus
}

fn main() {
    assert_eq!(
        Wizard {
            name: "Harry Potter".to_string(),
            age: 11,
            fav_spell: Spell::Expelliarmus
        },
        Wizard::eg()
    );
}
