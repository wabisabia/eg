use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
struct MyStruct {
    #[eg = "some other string"]
    string: String,
    #[eg = "1 + 3"]
    num: usize,
    #[eg = "func"]
    from_func: Vec<()>,
}

fn func() -> Vec<()> {
    vec![()]
}

#[test]
fn derive_struct() {
    assert_eq!(
        MyStruct {
            string: "some other string".to_string(),
            num: 4,
            from_func: vec![()],
        },
        MyStruct::eg()
    );
}

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
    WinguardiumLeviosa,
}

fn totally_random_spell() -> Spell {
    Spell::Expelliarmus
}

#[test]
fn readme_example() {
    assert_eq!(
        Wizard {
            name: "Harry Potter".to_string(),
            age: 11,
            fav_spell: Spell::Expelliarmus
        },
        Wizard::eg()
    )
}
