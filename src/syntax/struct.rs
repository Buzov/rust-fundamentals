#[derive(Default, Debug)]
pub(crate) struct Struct {
    x: f64,
    y: f64,
}

impl PartialEq<Struct> for Struct {
    fn eq(&self, other: &Struct) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub(crate) fn test_base_traits() {
    // Need to implement Default
    let s_1 = Struct::default();
    println!("s_1 {:?}", s_1);

    let s_2 = Struct::default();
    println!("s_2 {:?}", s_2);
    // Need to implement PartialEq
    let s_1_eq_s_2 = s_1 == s_2;
    println!("s_1 == s_2: {}", s_1_eq_s_2);
    let s_3 = Struct { x: 1.0, y: 2.0 };
    println!("s_3 {:?}", s_3);

    let s_1_eq_s_3 = s_1 == s_3;
    println!("s_1 == s_3: {}", s_1_eq_s_3);

    let x: f64 = 1.5;
    let y: f64 = 2.5;
    let s_4 = Struct { x, y };
    // Need yo implement Debug
    println!("s_4 {:?}", s_4);
}

