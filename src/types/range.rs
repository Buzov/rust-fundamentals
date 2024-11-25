pub(super)fn test_ranges() {
    let bounded: std::ops::Range<i32> = 0..10;
    let from = 0..;
    let to = ..10;
    let full = ..;
    let inclusive = 0..=9;

    for i in (0..10).step_by(2) {
        println!("i = {}", i);
    }
}