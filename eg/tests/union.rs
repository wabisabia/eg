use eg::Eg;

#[derive(Eg)]
#[cfg_attr(test, allow(dead_code))]
union IntOrFloat {
    #[eg]
    int: i32,
    float: f32,
}

#[test]
fn unnamed_union() {
    let default_union = IntOrFloat { int: i32::eg() };
    unsafe { assert_eq!(default_union.int, IntOrFloat::eg().int) }
}
