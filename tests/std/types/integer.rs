use ofin::std::Integer;

/// Shortcut macro to create an Integer
macro_rules! num {
    ($num: literal) => {
        Integer::from($num as i64)
    };
}

#[test]
fn addition() {
    assert_eq!(num!(2) + num!(2), num!(4));
    assert_eq!(
        num!(17757243711771519) + num!(659628482),
        num!(17757903340261)
    );
}
