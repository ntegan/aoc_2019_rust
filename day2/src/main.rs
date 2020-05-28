fn main() {
    let aa = a::get_ans_one();

    println!("Answer 1: {}", aa);
}

mod a {
    use std::fs;
    use super::Intcode;
    pub fn get_ans_one() -> i64 {
        let p1 = "1,1,1,4,99,5,6,0,99";
        let p2 ="1,9,10,3,2,3,11,0,99,30,40,50";
        let p3 = get_input();
        let mut p = Intcode::Program::fromstring(String::from(p3));
        p.pretty_print();
        p.evaluate();
        p.pretty_print();

        0
    }

    fn get_input() -> String {
        match fs::read_to_string("input.txt") {
            Ok(f) => { f },
            Err(_) => {
                match fs::read_to_string("day2/input.txt") {
                    Ok(fi) => { fi},
                    Err(_) => { String::from("no") }
                }
            }
        }
    }
}
// intcode program is a list of integers separated by commas
pub mod Intcode {
    // after do an opcode, step forward 4 positions
    pub struct Program {
        the_program: Vec<isize>,
    }
    impl Program {
        pub fn fromstring(s: String) -> Program {
            let int_list = s.split(",")
                .map(|f| { f.parse::<isize>().expect("Uh oh parsing program")})
                .collect::<Vec<isize>>();
            Program { the_program: int_list.clone() }
        }
        pub fn pretty_print(&self) {
            // 4 ints per line
            let mut j = 1;
            for i in 0..self.the_program.len() {
                print!("{},", self.the_program[i]);
                if j % 4 == 0 { println!("") } else {}
                if self.the_program[i] == 99 { println!("") } else {}

                j = j + 1;
            }
            println!("");
        }
        pub fn evaluate(&mut self) {
            let mut i = 0;
            let p = &mut self.the_program;
            loop {
                match p[i] {
                    1 => { // ADD
                        let dp = p[i + 3] as usize; // destination position
                        let sp1 = p[i + 1] as usize; // source pos 1
                        let sp2 = p[i + 2] as usize; // source pos 2
                        p[dp] = p[sp1] + p[sp2];
                        i += 4;
                    },
                    2 => { // MULTIPLY
                        let dp = p[i + 3] as usize; // destination position
                        let sp1 = p[i + 1] as usize; // source pos 1
                        let sp2 = p[i + 2] as usize; // source pos 2
                        p[dp] = p[sp1] * p[sp2];
                        i += 4;
                    },
                    99 => { break; }, // END OF PROGRAM
                    _ => { println!("HI: {}", i); panic!("invalid instr") }
                }
                if i >= p.len() { panic!("past end of prog") } else {}
            }
        }
    }
}
