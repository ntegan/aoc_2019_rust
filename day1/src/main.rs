fn main() {
    println!("Answer 1: {}", a::get_ans_one());
    println!("Answer 2: {}", a::get_ans_two());
}

mod a {
    use std::fs;
    use std::cmp::Ordering;

    pub fn get_ans_one() -> i64 { calculate_the_fuel(get_input())}
    pub fn get_ans_two() -> i64 { calculate_the_fuel_recursive(get_input())}
    pub fn fuel_for_mass(module_mass: i64) -> i64 { (module_mass / 3) - 2 }
    fn get_input() -> String {
        match fs::read_to_string("input.txt") {
            Ok(f) => { f },
            Err(_) => {
                match fs::read_to_string("day1/input.txt") {
                    Ok(fi) => { fi},
                    Err(_) => { String::from("no") }
                }
            }
        }
    }

    fn calculate_the_fuel(stringput: String) -> i64 {
        let mut total_fuel = 0i64;
        for line in stringput.lines() {
            match line.parse::<i64>() {
                Ok(i) => {total_fuel += fuel_for_mass(i);},
                Err(_e) => {},
            }
        }
        total_fuel
    }
    fn calculate_the_fuel_recursive(stringput: String) -> i64 {
        let mut total_fuel = 0i64;
        for line in stringput.lines() {
            match line.parse::<i64>() {
                Err(_e) => {},
                Ok(i) => {
                    let mut j = i;
                    loop {
                        j = fuel_for_mass(j);
                        if j <= 0 { break; } else {}
                        total_fuel += j;
                    }
                },
            }
        }
        total_fuel
    }

}
