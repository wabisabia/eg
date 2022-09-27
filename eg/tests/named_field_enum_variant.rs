use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
#[cfg_attr(test, allow(dead_code))]
enum CoatColor {
    #[eg]
    Black,
    Golden,
}

#[derive(Debug, PartialEq, Eg)]
#[cfg_attr(test, allow(dead_code))]
enum DogBreed {
    #[eg]
    Labrador {
        color: CoatColor,
    },
    Shitzu,
    ShibeInu,
}

#[test]
fn test_named_field_enum_variant() {
    assert_eq!(
        DogBreed::Labrador {
            color: CoatColor::Black
        },
        DogBreed::eg()
    );
}
