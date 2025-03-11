const TEXT: &str = "
1 green bottle2 hanging on the wall,
1 green bottle2 hanging on the wall,
And if one green bottle should accidentally fall,
There'll be 3 green bottle4 hanging on the wall.
";

const UPPER: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];
const LOWER: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut text = String::new();
    for i in 0..take_down {
        let count = start_bottles - i;
        let v1 = UPPER[count as usize];
        let v2 = if count == 1 { "" } else { "s" };
        let v3 = LOWER[(count - 1) as usize];
        let v4 = if count - 1 == 1 { "" } else { "s" };
        text += &TEXT
            .replace("1", v1)
            .replace("2", v2)
            .replace("3", v3)
            .replace("4", v4);
    }
    text
}
