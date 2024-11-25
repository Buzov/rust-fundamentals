
pub fn loops() {
    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} и j = {}", i, j);
    }

    let lines = "привет\nмир\nhello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    let x: () = while false {};
    let x2: () = if true {  92; };
    //    let x3 = if true {  92 }; error ожидается тип () если нет блока else

    //    while true  {
    //        if cond1 {
    //            continue;
    //        }
    //        if cond2 {
    //            break;
    //        }
    //    }
    //
    //    'outer: while cond1 {
    //        while cond2 {
    //            break 'outer;
    //        }
    //    }

}
pub fn test_loop() {
    //    let uninit;
    //    while true {
    //        if condition {
    //            uninit = 92;
    //            break;
    //        }
    //    }
    //    pritnln!("{}", uninit); // error
    let x = 30;
    let init;
    loop {
        if x == 31 {
            init = 92;
            break;
        }
    }
    println!("{}", init); // ok

}

pub fn test_loop_2() {
    let x = 30;
    let init;

    if x == 31 {
        init = 92;
    } else {
        loop {}
    }
    println!("{}", init); // ok!
}

pub fn test_loop_3() {
    let x = 30;
    let init: i32 = loop {
        if  x == 31 {
            break 92;
        }
    };

    println!("{}", init); // ok!
}

fn loop_test<T>() -> Box<T> {
    loop {
    }
}

fn exclamation_mark_0() -> i32 {
    let x: i32 = exclamation_mark_1();
    x
}

fn exclamation_mark_1() -> ! {
    //    let i = return;
    let x = exclamation_mark_2();
    x
}

fn exclamation_mark_2() -> ! {
    loop {
    }
}
