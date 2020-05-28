fn main() {
    let aa = a::get_ans_one();
    let ab = a::get_ans_two();

    println!("Answer 1: {}", aa);
    println!("Answer 2: {}", ab);
}

mod a {
    use std::fs;
    use super::Intcode;
    pub fn get_ans_one() -> i64 {
        let p1 = "1,1,1,4,99,5,6,0,99";
        let p2 ="1,9,10,3,2,3,11,0,99,30,40,50";
        let mut p3 = get_input();
        remove_whitespace(&mut p3);

        let mut p = Intcode::Program::fromstring(String::from(p3));

        p.problem_one()
    }
    pub fn get_ans_two() -> i64 {
        // what pair of inputs produces 19690720
        //  inputs replace addrs 1 and 2
        //                  noun    verb
        //                  each b/t 0 99 inclusive
        // output at addr 0
        //
        // make sure aech time try input, reset mem to beginning
        //
        // return 100 * noun * verb
        let mut prog_string = get_input();
        remove_whitespace(&mut prog_string);
        for i in 0..99+1 {
            for j in 0..99+1 {
                if Intcode::Program::new_evaluate_inputs(
                    prog_string.clone(), i, j) == 19690720 {
                    return 100 * i + j
                } else {}
            }
        }

        1
    }

      //s.retain(|c| !c.is_whitespace());
    fn remove_whitespace(s: &mut String) {
        s.retain(|c| !c.is_whitespace())
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
        pub fn new_evaluate_inputs(s: String, a: i64, b: i64) -> i64 {
            let mut prog = Program::fromstring(s);
            prog.the_program[1] = a as isize; prog.the_program[2] = b as isize;
            prog.evaluate();
            prog.the_program[0] as i64
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
        pub fn problem_one(&mut self) -> i64 {
            self.the_program[1] = 12;
            self.the_program[2] = 2;

            self.evaluate();

            self.the_program[0] as i64
        }
    }
}
