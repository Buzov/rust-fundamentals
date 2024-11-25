pub fn block() {
    let i: i32 = { 1  };
    let i: ()  = { 1; };
}

pub(super)fn test_expression() -> i32 {
    let x = 30;
    if x == 0 {
        println!("zero");
    }      // statement

    { 0; } // statement

    let s = if x > 0 {
        "positive"
    } else {
        "negative"
    };

    if true { 92 } else { 62 } // expression!
}
