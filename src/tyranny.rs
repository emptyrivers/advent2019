
fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn solve() {
    let data = super::get_data("01_tyranny.txt");
    let mut mass = 0;
    for word in data.iter() {
        let num = word.parse::<i32>().unwrap();
        mass += get_fuel(num);
    }
    println!("Total mass is {}", mass);
}