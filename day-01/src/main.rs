use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let list = input.split("\n");

    let mut prev_calories = 0;
    let mut current_calories = 0;
    let mut highest_calories = 0;

    list.for_each(|item| {
        let current_is_greater = current_calories > prev_calories;

        if item != "" {
            let value = item.parse::<i32>().unwrap();

            current_calories += value;

            if current_is_greater {
                highest_calories = current_calories;
            }
        } else {
            if current_is_greater {
                prev_calories = current_calories;
            }

            current_calories = 0;
        }
    });

    println!("{}", highest_calories);
}
