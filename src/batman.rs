use crate::riddler;

fn my_real_name() -> String {
    "Bruce Wayne".to_string()
}

pub fn batman_quote() -> String {
    // nobody will know
    let _my_secret_identity = my_real_name();

    // ask riddler for a riddle
    let _riddle = riddler::difficult_riddle();

    "Robin, you haven't fastened your safety bat-belt.".to_string()
}

pub fn advice() -> String {
    "You've made a hasty generalization, Robin. It's a bad habit to get into.".to_string()
}
