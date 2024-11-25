mod types;
mod std;

use crate::types::number::number_type;
use crate::types::string::string_slice;
use crate::types::string::owned_string;
use crate::types::string::conversion_between_string_and_str;
use crate::types::string::c_string;
use crate::types::string::os_string;
use crate::types::string::cow_string;

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

}
