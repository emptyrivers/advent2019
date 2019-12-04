
pub fn solve() {
    let mut data = super::get_data_as_ints("02_opcodes.txt");
    data[1] = 12;
    data[2] = 2;
    for i in 0..data.len()/4 {
        let index = i * 4;
        println!("op: {}, data: [{}, {}], loc: {}", data[index], data[index+1], data[index+2], data[index+3]);
        match data[index] {
            99 => {
                println!("done!")
            },
            1 => {
                let loc0 = data[index + 1] as usize;
                let loc1 = data[index + 2] as usize;
                let loc = data[index + 3] as usize;
                data[loc] = data[loc0] + data[loc1];
                println!("result: {}", data[loc])
            },
            2 => {
                let loc0 = data[index + 1] as usize;
                let loc1 = data[index + 2] as usize;
                let loc = data[index + 3] as usize;
                data[loc] = data[loc0] * data[loc1];
                println!("result: {}", data[loc])
            },
            _ => panic!("invalid opcode {}", data[index])
        }
    }
    println!("Part 1: final result is {}", data[0])
}