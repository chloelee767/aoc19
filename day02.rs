use std::fs;

fn main() {
    part_2();
}

fn part_2() {
    let original = parse_input("day02.txt");
    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut current = original.clone();
            current[1] = noun;
            current[2] = verb;
            run_program(&mut current);
            if current[0] == target {
                println!("noun: {}, verb: {}", noun, verb);
                println!("100 * noun + verb = {}", (100 * noun) + verb);
                break;
            }
        }
    }
}

fn part_1() {
    let mut ints = parse_input("day02.txt"); // [1,9,10,3,2,3,11,0,99,30,40,50];
    ints[1] = 12;
    ints[2] = 2;
    println!("{:?}", ints);
    run_program(&mut ints);
    println!("{:?}", ints);
}

fn run_program(ints: &mut [u32]) {
    let mut i: usize = 0;
    while i < ints.len() && ints[i] != 99 {
        if ints[i] == 1 {
            opcode_1(ints, &i);
        } else if ints[i] == 2 {
            opcode_2(ints, &i);
        }

        i += 4;
    }
}

fn opcode_1(ints: &mut [u32], index: &usize) {
    operate(ints, index, &|x, y| x + y);
}

fn opcode_2(ints: &mut [u32], index: &usize) {
    operate(ints, index, &|x, y| x * y);
}

fn operate(ints: &mut [u32], index: &usize, f: &dyn Fn(u32, u32) -> u32) {
    let index1 = ints[*index + 1] as usize;
    let index2 = ints[*index + 2] as usize;
    let index3 = ints[*index + 3] as usize;
    let result = f(ints[index1], ints[index2]);
    ints[index3] = result;
}

fn parse_input(filename: &str) -> Vec<u32> {
    let raw = fs::read_to_string(filename).unwrap();
    println!("{}", raw);
    let mut v: Vec<u32> = vec![];
    for digit in raw.split(',') {
        // println!("{}", digit);
        let x = digit.parse::<u32>();
        if x.is_ok() {
            v.push(x.unwrap());
        }
    }
    // let mut v : Vec<u32> = raw.split(',').map(|x| x.parse::<u32>().expect(x)).collect();
    return v;
}
