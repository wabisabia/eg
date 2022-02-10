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
