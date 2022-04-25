use std::{cmp, env, fs};

/// Gets the names of the files to be compared from the command line.
fn get_filenames() -> (String, String) {
    let files: Vec<String> = env::args().collect();

    if files.len() < 3_usize {
        panic!("Expected 2 arguments, got {}", files.len() - 1);
    }

    (files[1].clone(), files[2].clone())
}

/// Reads the given file and outputs it line by line.
fn read_file_lines(filename: String) -> Vec<String> {
    let file: String = fs::read_to_string(filename).expect("Couldn't read file ");
    file.lines().map(|l| l.to_string()).collect()
}

/// Builds a grid with the longest common subsequence algorithm.
fn longest_common_subsequence(file1: &[String], file2: &[String]) -> Vec<Vec<usize>> {
    let file1_len = &file1.len();
    let file2_len = &file2.len();

    let mut result_grid = vec![vec![0; *file2_len + 1]; *file1_len + 1];

    for (i, f1_line) in file1.iter().enumerate() {
        for (j, f2_line) in file2.iter().enumerate() {
            if f1_line == f2_line {
                result_grid[i + 1][j + 1] = result_grid[i][j] + 1;
            } else {
                result_grid[i + 1][j + 1] = cmp::max(result_grid[i + 1][j], result_grid[i][j + 1])
            }
        }
    }

    result_grid
}

/// Prints out the difference between the two given vectors of strings, using their previously
/// calculated largest common subsequence grid.
fn print_diff(subsequence_coparison_grid: &Vec<Vec<usize>>, file1: &[String], file2: &[String], file1_iter: usize, file2_iter: usize) {
    match (file1_iter, file2_iter) {
        (file1_iter, file2_iter) if file1_iter > 0 && file2_iter > 0 && file1[file1_iter - 1] == file2[file2_iter - 1] => {
            print_diff(subsequence_coparison_grid, file1, file2, file1_iter  - 1, file2_iter - 1);
            println!("{}", file1[file1_iter  - 1]);
        },
        (file1_iter , file2_iter) if file2_iter > 0 && (file1_iter  == 0 || subsequence_coparison_grid[file1_iter ][file2_iter - 1] >= subsequence_coparison_grid[file1_iter  - 1][file2_iter]) => {
            print_diff(subsequence_coparison_grid, file1, file2, file1_iter , file2_iter - 1);
            println!("> {}", file2[file2_iter - 1]);
        },
        (file1_iter , file2_iter) if file1_iter  > 0 && (file2_iter == 0 || subsequence_coparison_grid[file1_iter ][file2_iter - 1] < subsequence_coparison_grid[file1_iter  - 1][file2_iter]) => {
            print_diff(subsequence_coparison_grid, file1, file2, file1_iter  - 1, file2_iter);
            println!("< {}", file1[file1_iter  - 1]);
        },
        _ => println!(),
    };
}

fn main() {
    let (filename1, filename2) = get_filenames();
    let file1 = read_file_lines(filename1);
    let file2 = read_file_lines(filename2);
    let grid = longest_common_subsequence(&file1, &file2);

    print_diff(&grid, &file1, &file2, file1.len(), file2.len());
}
