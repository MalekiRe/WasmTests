#[no_mangle]
pub extern "C" fn test_function() {
    println!("This will trigger the bug");
}
