use sets::{Set, MutSetOps};
use std::fs::read_to_string;

fn main()
{
    // Problem 1
    println!("Problem 1: {}", (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<i32>());

    // Problem 2
    struct Fibonacci {
        a: i64,
        b: i64,
    }
    impl Iterator for Fibonacci {
        type Item = i64;
        fn next(&mut self) -> Option<i64> {
            let result = self.a + self.b;
            self.a = self.b;
            self.b = result;
            Some(result)
        }
    }
    let fibonacci = Fibonacci { a: 1, b: 1 };
    println!("Problem 2: {}", fibonacci.take_while(|f| f <= &4000000).filter(|x| x % 2 == 0).sum::<i64>());

    // Problem 31
    let coins: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    fn combinations(coins: &[usize], remainder: usize) -> usize {
        if coins.is_empty() {
            if remainder == 0 {
                1
            } else {
                0
            }
        } else {
            let mut result = 0;
            for _i in (0..remainder + 1).step_by(coins[0]) {
                result += combinations(&coins[1..], remainder - _i);
            }
            result
        }
    }
    println!("Problem 31: {}", combinations(&coins, 200));

    // Problem 96
    struct Sudoku {
        rows: Vec<Set<usize>>,
        cols: Vec<Set<usize>>,
        boxes: Vec<Set<usize>>,
    }

    impl Sudoku {
        fn new() -> Self {
            let mut rows: Vec<Set<usize>> = Vec::new();
            let mut cols: Vec<Set<usize>> = Vec::new();
            let mut boxes: Vec<Set<usize>> = Vec::new();
            for i in 0..9 {
                rows.push(Set::new_empty());
                cols.push(Set::new_empty());
                boxes.push(Set::new_empty());
                for v in 1..10 {
                    rows[i].minsert(v);
                    cols[i].minsert(v);
                    boxes[i].minsert(v);
                }
            }
            Sudoku {
                rows: rows,
                cols: cols,
                boxes: boxes,
            }
        }
        fn indices(&self, x: usize, y: usize) -> (usize, usize, usize) {
            (x, y, x / 3 + y / 3 * 3)
        }
        fn remaining(&self, x: usize, y: usize) -> Set<usize> {
            let (row, col, box_) = self.indices(x, y);
            return self.rows[row].intersection(&self.cols[col]).intersection(&self.boxes[box_]);
        }
    }
    let sudoku = Sudoku::new();
    println!("{:?}", sudoku.indices(0, 3));
    // read first line from sudoku.txt file
    let file = read_to_string("sudoku.txt");
    let binding = file.expect("Could not open file");
    let mut lines = binding.lines();
    let line = lines.next().expect("No lines in file").to_string();
    println!("{}", line);
}
