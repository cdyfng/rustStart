
pub fn function() {
    println!("called `nested::function() from nested.rs`");
}


#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}