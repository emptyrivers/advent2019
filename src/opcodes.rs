
fn evaluate(data: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut mem = data.clone();
    mem[1] = noun;
    mem[2] = verb;
    let mut ptr = 0;
    while ptr < mem.len() {
        let op = mem[ptr];
        match op {
            99 => return mem[0],
            1 => {
                let loc0 = mem[ptr + 1] as usize;
                let loc1 = mem[ptr + 2] as usize;
                let loc = mem[ptr + 3] as usize;
                mem[loc] = mem[loc0] + mem[loc1];
            },
            2 => {
                let loc0 = mem[ptr + 1] as usize;
                let loc1 = mem[ptr + 2] as usize;
                let loc = mem[ptr + 3] as usize;
                mem[loc] = mem[loc0] * mem[loc1];
            },
            _ => panic!("invalid opcode {}", mem[ptr])
        }
        ptr += 4;
    }
    panic!("Reached end of memory without stopping")
}


pub fn solve() {
    let data = super::get_data_as_ints("02_opcodes.txt");
    println!("Part 1: Result is {}", evaluate(&data, 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            match evaluate(&data, noun, verb) {
                19690720 => {
                    println!("Done. Noun = {}, Verb = {}", noun, verb);
                    break
                },
                _ => continue,
            }
        }
    }
}