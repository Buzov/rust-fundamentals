//Zero Sized Types
//unit struct
struct Zero;

pub fn zero_sized_types() -> () {
    let t = Zero;
    println!("struct Zero;");

    println!("std::mem::size_of::<Zero>() = {}", std::mem::size_of::<Zero>());
    assert!(std::mem::size_of::<Zero>() == 0);

    println!("std::mem::size_of::<(Zero, Zero)>() = {}", std::mem::size_of::<(Zero, Zero)>());
    assert!(std::mem::size_of::<(Zero, Zero)>() == 0);

    println!("std::mem::size_of::<[Zero; 1024]>() = {}", std::mem::size_of::<[Zero; 1024]>());
    assert!(std::mem::size_of::<[Zero; 1024]>() == 0);

    println!("std::mem::size_of::<()>() = {}", std::mem::size_of::<()>());
    assert!(std::mem::size_of::<()>() == 0);
}
