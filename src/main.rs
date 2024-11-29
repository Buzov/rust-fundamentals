mod types;
mod std;
mod syntax;
mod sequences_and_maps;
mod memory;
mod smart_pointer;

use crate::types::number::number_type;
use crate::types::string::string_slice;
use crate::types::string::owned_string;
use crate::types::string::conversion_between_string_and_str;
use crate::types::string::c_string;
use crate::types::string::os_string;
use crate::types::string::cow_string;
use crate::types::tuple::tuple;
use crate::memory::zero_struct::zero_sized_types;
use crate::syntax::r#struct::test_base_traits;

use crate::smart_pointer::box_pointer::run_truck;
use crate::smart_pointer::rc_pointer::run_rc;
use crate::smart_pointer::arc_pointer::run_arc;

fn main() {

    println!("---Numbers----");
    number_type();

    println!("\n---Strings----");
    string_slice();
    owned_string();
    conversion_between_string_and_str();
    c_string();
    os_string();
    cow_string();

    println!("\n---Tuples----");
    tuple();

    println!("\n---Memory----");
    zero_sized_types();

    println!("\n---Struct----");
    test_base_traits();

    println!("\n---Smart Pointer----");
    run_truck();
    run_rc();
    run_arc();
}
