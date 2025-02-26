// For want of a nail the shoe was lost.
// For want of a shoe the horse was lost.
// For want of a horse the rider was lost.
// For want of a rider the message was lost.
// For want of a message the battle was lost.
// For want of a battle the kingdom was lost.
// And all for the want of a nail.

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new(); // Return an empty string for an empty list
    }

    let mut proverb = String::new();

    // Iterate over the list, constructing the proverb
    for i in 0..list.len() - 1 {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]));
    }

    // Handle the last item in the list
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
