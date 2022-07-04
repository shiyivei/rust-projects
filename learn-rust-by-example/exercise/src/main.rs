panic = 'abort'

use std::panic;


fn main() {

    let result = panic::catch_unwind(|| {
        panic!("oh no");
    });
    
}
