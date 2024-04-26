static SPACER: [i64; 1000] = [0; 1000];

#[no_mangle]
pub extern "C" fn test_function2() {
    let map: std::collections::HashMap<(), ()> = std::collections::HashMap::new();
}
