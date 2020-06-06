use replace_adjacent_duplicates::replace_duplicate;
fn main() {
    let text1 = "This is a duplicate of a duplicate duplicate duplicated!";
    println!("{}", replace_duplicate(text1));
}
