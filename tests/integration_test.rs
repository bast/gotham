#[test]
fn batman() {
    let quote = gotham::batman_quote();
    assert_eq!(quote, "Robin, you haven't fastened your safety bat-belt.");
}

#[test]
fn joker() {
    let quote = gotham::joker_warning();
    assert_eq!(
        quote,
        "And now people of Gotham City, the moment you have all been waiting for."
    );
}

#[test]
fn robin() {
    let quote = gotham::robin_exclamation();
    assert_eq!(quote, "Holy Bat Logic!");
}
