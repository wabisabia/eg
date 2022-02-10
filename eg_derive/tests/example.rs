use eg::Eg;

#[test]
fn named_struct() {
    #[derive(Debug, Eg, PartialEq)]
    struct MyStruct {
        string: String,
        unsigned_int: usize,
        signed_int: isize,
        float: f64,
    }

    let my_struct = MyStruct {
        string: "foobar".to_string(),
        unsigned_int: 42,
        signed_int: -1,
        float: 3.1415,
    };
    assert_eq!(my_struct, MyStruct::eg())
}

#[test]
fn unnamed_struct() {
    #[derive(Debug, Eg, PartialEq)]
    struct MyStruct(String, usize, isize, f64);

    let my_struct = MyStruct("foobar".to_string(), 42, -1, 3.1415);
    assert_eq!(my_struct, MyStruct::eg())
}

#[test]
fn unit_struct() {
    #[derive(Debug, Eg, PartialEq)]
    struct MyStruct;

    assert_eq!(MyStruct, MyStruct::eg())
}
