use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
struct MyStruct;

#[test]
fn unit_struct() {
    assert_eq!(MyStruct, MyStruct::eg())
}
