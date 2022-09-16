use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
struct MyStruct(String, usize, isize, f64);

#[test]
fn unnamed_struct() {
    let my_struct = MyStruct("foobar".to_string(), 42, -1, 3.1415);
    assert_eq!(my_struct, MyStruct::eg())
}
