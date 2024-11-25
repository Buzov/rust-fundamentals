pub fn tuple() {
    let pair: (f32, i32) = (0.0, 92);
    let one: (f32,) = (0.0,); // tuple of one element, comma required
    //    let (x, y) = pair;
    let (_x, _y) = pair;
    let x = pair.0;
    let y = pair.1;
    println!("{}", x);
    println!("{}", y);

    let t = (92,); // The tuple is worthless. The wrapper and element addresses are the same
    println!("{:?}", &t as *const (i32,)); // 0x7ffc6b2f6aa4
    println!("{:?}", &t.0 as *const i32); // 0x7ffc6b2f6aa4
}