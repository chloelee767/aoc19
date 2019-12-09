use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// try reading from standard input next

fn main() {
    solve_part2();
}

fn solve_part2() {
    let f = File::open("day03.txt").unwrap();
    let f = BufReader::new(f);
    let lines : Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    part2(&lines[0], &lines[1]);
}

#[allow(dead_code)]
fn part2_tests() {
    assert_eq!(30, part2("R8,U5,L5,D3", "U7,R6,D4,L4"));
    assert_eq!(610, part2("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(410, part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                          "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
}

fn part2(s1: &str, s2: &str) -> u32 {
    let wire1: Vec<HVLine> = parse_input(s1);
    let wire2: Vec<HVLine> = parse_input(s2);
    let mut steps: Vec<u32> = vec![];
    for (i1, l1) in wire1.iter().enumerate() {
        for (i2, l2) in wire2.iter().enumerate() {
            let pt = l1.intersect(&l2);
            if pt.is_some() {
                let pt = pt.unwrap();
                steps.push(pt.steps_to_origin(&wire1, i1) + pt.steps_to_origin(&wire2, i2));
                // cannot break here
            }
        }
    }

    steps.sort_unstable();
    println!("Least number of steps: {}", steps[1]);
    return steps[1];
}

#[allow(dead_code)]
fn solve_part1() {
    let f = File::open("day03.txt").unwrap();
    let f = BufReader::new(f);
    let lines : Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    part1(&lines[0], &lines[1]);
}

#[allow(dead_code)]
fn part1_tests() {
    assert_eq!(6, part1("R8,U5,L5,D3", "U7,R6,D4,L4"));
    assert_eq!(159, part1("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(135, part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                          "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
}

fn part1(s1: &str, s2: &str) -> u32 {
    let wire1: Vec<HVLine> = parse_input(s1);
    let wire2: Vec<HVLine> = parse_input(s2);
    let mut distances: Vec<u32> = vec![];
    for l1 in wire1.iter() {
        for l2 in wire2.iter() {
            let pt = l1.intersect(&l2);
            if pt.is_some() {
                distances.push(pt.unwrap().distance_to_origin());
            }
        }
    }
    distances.sort_unstable();
    println!("Shortest distance is {}", distances[1]); // exclude origin
    return distances[1];
}

fn parse_input(s: &str) -> Vec<HVLine> {
    let directions: Vec<Direction> = s
        .split(',')
        .map(|x| x.parse::<Direction>().unwrap())
        .collect();
    let mut point = Point { x: 0, y: 0 };
    let mut lines: Vec<HVLine> = vec![];
    for d in directions {
        let (li, pt) = parse_direction(&d, &point);
        point = pt;
        lines.push(li);
    }
    return lines;
}

#[derive(Copy,Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct Direction {
    d: Dir,
    magnitude: u32,
}

#[derive(Debug)]
struct DirectionParseError(String);

impl fmt::Display for DirectionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DirectionParseError {}

impl FromStr for Direction {
    type Err = DirectionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let dir = chars.next();
        let magn_result = chars.as_str().parse::<u32>();
        if magn_result.is_err() {
            return Err(DirectionParseError("Invalid magnitude".to_string()));
        } else {
            let magn = magn_result.unwrap();
            match dir {
                Some('L') => Ok(Direction {
                    d: Dir::Left,
                    magnitude: magn,
                }),
                Some('R') => Ok(Direction {
                    d: Dir::Right,
                    magnitude: magn,
                }),
                Some('U') => Ok(Direction {
                    d: Dir::Up,
                    magnitude: magn,
                }),
                Some('D') => Ok(Direction {
                    d: Dir::Down,
                    magnitude: magn,
                }),
                _ => Err(DirectionParseError("Invalid direction".to_string())),
            }
        }
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Manhattan distance to origin
    fn distance_to_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn steps_to_origin(&self, wire: &[HVLine], index: usize) -> u32 {
        wire[index].distance_from_start(self) +
            if index > 0 { wire[..index].into_iter().map(|l| l.magnitude).sum() } else { 0 }
    }
}

enum Orientation {
    Right,
    Up,
}

struct HVLine {
    orientation: Orientation,
    point: Point,
    magnitude: u32,
    original_dir: Dir
}

impl HVLine {
    fn new(orientation: Orientation, point: Point, magnitude: u32, original_dir: Dir) -> HVLine {
        HVLine {
            orientation,
            point,
            magnitude,
            original_dir,
        }
    }

    fn point1(&self) -> Point {
        self.point.clone()
    }

    fn point2(&self) -> Point {
        match self.orientation {
            Orientation::Right => Point {
                x: self.point.x + self.magnitude as i32,
                y: self.point.y,
            },
            Orientation::Up => Point {
                x: self.point.x,
                y: self.point.y + self.magnitude as i32,
            },
        }
    }

    fn start_point(&self) -> Point {
        match self.original_dir {
            Dir::Left | Dir::Down => self.point2(),
            Dir::Right | Dir::Up => self.point1(),
        }
    }

    fn intersect(&self, other: &HVLine) -> Option<Point> {
        let p1 = self.point1();
        let p2 = self.point2();
        let p3 = other.point1();
        let p4 = other.point2();

        let x: Option<i32> = intersect_helper(p1.x, p2.x, p3.x, p4.x);

        let y: Option<i32> = intersect_helper(p1.y, p2.y, p3.y, p4.y);

        if x.is_some() && y.is_some() {
            return Some(Point {
                x: x.unwrap(),
                y: y.unwrap(),
            });
        } else {
            return None;
        }
    }

    fn distance_from_start(&self, point: &Point) -> u32 {
        let start = self.start_point();
        return ((start.x - point.x).abs() + (start.y - point.y).abs()) as u32;
    }
}

fn intersect_helper(coord1: i32, coord2: i32, coord3: i32, coord4: i32) -> Option<i32> {
    return if coord1 <= coord3 && coord3 <= coord2 {
        Some(coord3)
    } else if coord1 <= coord4 && coord4 <= coord2 {
        Some(coord4)
    } else if coord3 <= coord1 && coord1 <= coord4 {
        Some(coord1)
    } else if coord3 <= coord2 && coord2 <= coord4 {
        Some(coord2)
    } else {
        None
    };
}

fn parse_direction(direction: &Direction, current: &Point) -> (HVLine, Point) {
    let magn = direction.magnitude;

    let pt2: Point;
    let line: HVLine;

    match direction.d {
        Dir::Left => {
            pt2 = Point {
                x: current.x - magn as i32,
                y: current.y,
            };
            line = HVLine::new(Orientation::Right, pt2.clone(), magn, direction.d);
        }
        Dir::Right => {
            pt2 = Point {
                x: current.x + magn as i32,
                y: current.y,
            };
            line = HVLine::new(Orientation::Right, current.clone(), magn, direction.d);
        }
        Dir::Up => {
            pt2 = Point {
                x: current.x,
                y: current.y + magn as i32,
            };
            line = HVLine::new(Orientation::Up, current.clone(), magn, direction.d);
        }
        Dir::Down => {
            pt2 = Point {
                x: current.x,
                y: current.y - magn as i32,
            };
            line = HVLine::new(Orientation::Up, pt2.clone(), magn, direction.d);
        }
    }

    return (line, pt2);
}
