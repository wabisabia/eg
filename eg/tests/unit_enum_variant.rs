use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
#[cfg_attr(test, allow(dead_code))]
enum DogBreed {
    #[eg]
    Labrador,
    Shitzu,
    ShibeInu,
}

#[test]
fn test_unit_enum() {
    assert_eq!(DogBreed::Labrador, DogBreed::eg());
}
