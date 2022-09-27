use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
#[cfg_attr(test, allow(dead_code))]
enum CoatColor {
    Black,
    Golden,
}

#[derive(Debug, PartialEq, Eg)]
#[cfg_attr(test, allow(dead_code))]
enum DogBreed {
    #[eg]
    Labrador(CoatColor),
    Shitzu,
    ShibeInu,
}

#[test]
fn test_tuple_enum_variant() {
    assert_eq!(DogBreed::Labrador(CoatColor::Black), DogBreed::eg());
}
