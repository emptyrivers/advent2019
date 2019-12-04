
fn get_fuel(mass: i64) -> i64 {
    ((mass / 3) - 2).abs()
}

pub fn solve() {
    let data = super::get_data_as_ints("01_tyranny.txt");
    let mut mass = 0;
    let mut supermass = 0;
    for &num in data.iter() {
        let mut rem_mass = get_fuel(num);
        mass += rem_mass;
        supermass += rem_mass;
        while rem_mass >= 9 {
            rem_mass = get_fuel(rem_mass);
            supermass += rem_mass;
        }
    }
    println!("Part 1: Total mass is {}", mass);
    println!("Part 2: Super total is {}", supermass)
}