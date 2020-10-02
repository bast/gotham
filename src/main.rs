mod riddler;

fn main() {
    println!("Na na na na na na na na Batman!");

    // using a library function
    let quote = gotham::batman_quote();
    println!("Batman says: {}", quote);

    // this function is not part of the library
    let quote = riddler::difficult_riddle();
    println!("Riddler says: {}", quote);
}
