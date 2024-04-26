use std::cell::OnceCell;
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn test_function() {
    const A: f32 = 1.0;
    static mut B: f32 = 1.0;

    let map: HashMap<i32, i32> = HashMap::new();
}

