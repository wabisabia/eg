use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
struct UntaggedStruct {
    string: String,
    unsigned_int: usize,
    signed_int: isize,
    float: f64,
}

#[test]
fn untagged() {
    let my_struct = UntaggedStruct {
        string: "foobar".to_string(),
        unsigned_int: 42,
        signed_int: -1,
        float: 3.1415,
    };
    assert_eq!(my_struct, UntaggedStruct::eg())
}

#[derive(Debug, PartialEq, Eg)]
struct TaggedStruct {
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
fn tagged_with_fn() {
    assert_eq!(
        TaggedStruct {
            string: "some other string".to_string(),
            num: 4,
            from_func: vec![()],
        },
        TaggedStruct::eg()
    );
}
