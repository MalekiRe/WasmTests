#![feature(raw_ref_op)]

#[no_mangle]
pub extern "C" fn test_function2() {
    let map: std::collections::HashMap<(), ()> = std::collections::HashMap::new();
}
