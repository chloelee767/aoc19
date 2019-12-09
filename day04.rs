use std::cmp::{max, Eq};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let min_pw = &args[1];
    let max_pw = &args[2];
    let print = &args[3];
    let predicate = has_exactly_2_adjacent; // has_adjacent;
    let n = num_passwords(
        min_pw.parse().unwrap(),
        max_pw.parse().unwrap(),
        print.parse().unwrap(),
        &predicate,
    );
    println!(
        "Number of passwords between {} and {} : {}",
        min_pw, max_pw, n
    );
}

#[allow(dead_code)]
fn test_exactly_2() {
    assert_eq!(has_exactly_2_adjacent(&[1, 1]), true);
    assert_eq!(has_exactly_2_adjacent(&[1, 1, 1]), false);
    assert_eq!(has_exactly_2_adjacent(&[1, 1, 1, 2, 2]), true);
    assert_eq!(has_exactly_2_adjacent(&[1, 1, 1, 2, 2, 2]), false);
    assert_eq!(has_exactly_2_adjacent(&[1, 1, 1, 2, 2, 1]), true);
    assert_eq!(has_exactly_2_adjacent(&[1, 1, 2, 2, 2]), true);
}

fn has_exactly_2_adjacent<T: Eq>(arr: &[T]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] == arr[i + 1]
            && (i == 0 || arr[i - 1] != arr[i])
            && (i == arr.len() - 2 || arr[i + 1] != arr[i + 2])
        {
            return true;
        }
    }
    return false;
}

#[allow(dead_code)]
fn has_adjacent<T: Eq>(arr: &[T]) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] == arr[i + 1] {
            return true;
        }
    }
    return false;
}

fn digit(num: u32, index: usize) -> u32 {
    let chars: Vec<char> = num.to_string().chars().collect();
    return chars[index].to_digit(10).unwrap();
}

fn make_num(digits: &[u32]) -> u32 {
    let mut num = 0;
    for index in 0..digits.len() {
        let i = digits.len() - index - 1;
        let d = digits[i];
        num += d * 10u32.pow(index as u32);
    }
    return num;
}

// TODO find a nicer way to do this that can work with passwords of generic length
fn num_passwords(min_pw: u32, max_pw: u32, print: bool, predicate: &dyn Fn(&[u32]) -> bool) -> u32 {
    let mut count = 0;
    'outer: for d0 in digit(min_pw, 0)..10 {
        for d1 in max(d0, 0)..10 {
            for d2 in max(d1, 0)..10 {
                for d3 in max(d2, 0)..10 {
                    for d4 in max(d3, 0)..10 {
                        for d5 in max(d4, 0)..10 {
                            let arr = [d0, d1, d2, d3, d4, d5];
                            let num = make_num(&arr);
                            if num > max_pw {
                                if print {
                                    println!("Skipped {}", num);
                                }
                                break 'outer;
                            }
                            if num >= min_pw && predicate(&arr) {
                                if print {
                                    println!("{}", num);
                                }
                                count += 1;
                            } else if print {
                                println!("Skipped {}", num);
                            }
                        }
                    }
                }
            }
        }
    }
    return count;
}
