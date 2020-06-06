use replace_adjacent_duplicates::replace_duplicate;
fn main() {
    let text1 = "This is a duplicate of a duplicate duplicate duplicated!";
    let text2 = "This is is something";

    println!("{}", replace_duplicate(text1));
    println!("{}", replace_duplicate(text2));
}
