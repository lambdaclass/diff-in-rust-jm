use std::{cmp, env, fs};

fn get_filenames() -> (String, String) {
    let files: Vec<String> = env::args().collect();

    if files.len() < 3_usize {
        panic!("Expected 2 arguments, got {}", files.len() - 1);
    }

    (files[1].clone(), files[2].clone())
}

fn read_file_lines(filename: String) -> Vec<String> {
    let file: String = fs::read_to_string(filename).expect("Couldn't read file ");
    file.lines().map(|l| l.to_string()).collect()
}

fn lcs(f1: &[String], f2: &[String]) -> Vec<Vec<usize>> {
    let m = &f1.len();
    let n = &f2.len();

    let mut c = vec![vec![0; *n + 1]; *m + 1];

    for (i, f1_line) in f1.iter().enumerate() {
        for (j, f2_line) in f2.iter().enumerate() {
            if f1_line == f2_line {
                c[i + 1][j + 1] = c[i][j] + 1;
            } else {
                c[i + 1][j + 1] = cmp::max(c[i + 1][j], c[i][j + 1])
            }
        }
    }

    c
}

fn print_diff(c: &Vec<Vec<usize>>, f1: &[String], f2: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && f1[i - 1] == f2[j - 1] {
        print_diff(c, f1, f2, i - 1, j - 1);
        println!("{}", f1[i - 1]);
    } else if j > 0 && (i == 0 || c[i][j - 1] >= c[i - 1][j]) {
        print_diff(c, f1, f2, i, j - 1);
        println!("> {}", f2[j - 1]);
    } else if i > 0 && (j == 0 || c[i][j - 1] < c[i - 1][j]) {
        print_diff(c, f1, f2, i - 1, j);
        println!("< {}", f1[i - 1]);
    } else {
        println!();
    }
}

fn main() {
    let (filename1, filename2) = get_filenames();
    let f1 = read_file_lines(filename1);
    let f2 = read_file_lines(filename2);
    let c = lcs(&f1, &f2);

    print_diff(&c, &f1, &f2, f1.len(), f2.len());
}
