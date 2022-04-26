use std::{cmp, env, fs};

/// The program gets the two files it will process from the command line.
/// Then it applies the longest_common_subsequence function. This function creates a table of
/// comparisons that eventually gives print_diff a path to follow when printing. While following
/// this path, print_diff will let us know how the two files differ and coincide. This is shown
/// with the symbols '<' for the first file, '>' for the second file or nothing if it's a
/// coincidence.
fn main() {
    let (filename1, filename2) = get_filenames();
    let file1 = read_file_lines(filename1);
    let file2 = read_file_lines(filename2);
    let grid = longest_common_subsequence(&file1, &file2);

    print_diff(&grid, &file1, &file2, file1.len(), file2.len());
}

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
/// Given two files (represented as vectors of strings) the subsequence checks for order but not
/// for continuity. This means the two sequences 'a b c d' and 'a c d e' will have 'a c d' as their
/// longest common subsequence. The way this is calculated is by building a grid like so:
///
/// First is to make a double entry table with a padding (this padding will be used to end the
/// looping)
/// ```
///  | |a|b|c|d
///  |0|0|0|0|0
/// a|0| | | |
/// c|0| | | |
/// d|0| | | |
/// e|0| | | |
/// ```
///
/// Then the entries are compared, iterating each row through every column, and if a coincidence is
/// found, a 1 is added to the cell. In every iteration, the top and the left cells are
/// compared and the bigger of the two values is added to this coincidence cell.
/// ```
///  | |a|b|c|d
///  |0|0|0|0|0
/// a|0|1|1|1|1
/// c|0|1|1|2|2
/// d|0|1|1|2|3
/// e|0|1|1|2|3
/// ```
///
/// This table is then paired with print_diff, which will traverse it and find the difference.
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
/// The following example shows how the grid is traversed, starting by the bottom-right corner:
/// ```
///  | |a|b|c|d     | |a|b|c|d     | |a|b|c|d     | |a|b|c|d     | |a|b|c|d     | |a|b|c|d
///  |0|0|0|0|0     |0|0|0|0|0     |0|0|0|0|0     |0|0|0|0|0     |0|0|0|0|0     |0|X|0|0|0
/// a|0|1|1|1|1    a|0|1|1|1|1    a|0|1|1|1|1    a|0|1|1|1|1    a|0|1|X|1|1    a|0|1|X|1|1
/// c|0|1|1|2|2 -> c|0|1|1|2|2 -> c|0|1|1|2|2 -> c|0|1|1|X|2 -> c|0|1|1|X|2 -> c|0|1|1|X|2
/// d|0|1|1|2|3    d|0|1|1|2|3    d|0|1|1|2|X    d|0|1|1|2|X    d|0|1|1|2|X    d|0|1|1|2|X
/// e|0|1|1|2|3    e|0|1|1|2|X    e|0|1|1|2|X    e|0|1|1|2|X    e|0|1|1|2|X    e|0|1|1|2|X
/// ```
///
/// After positioning the start, the function checks if for the max between the top and left cell,
/// moving the iteration to the max of this two values. If both values are smaller than the value
/// at the current cell, this means a match was found and the iteration moves diagonally; while
/// adding the string to the largest common subsequence.
fn print_diff(subsequence_comparison_grid: &Vec<Vec<usize>>, file1: &[String], file2: &[String], file1_iter: usize, file2_iter: usize) {
    if file1_iter > 0 && file2_iter > 0 && file1[file1_iter - 1] == file2[file2_iter - 1] {
        print_diff(subsequence_comparison_grid, file1, file2, file1_iter  - 1, file2_iter - 1);
        println!("{}", file1[file1_iter  - 1]);
    } else
    if file2_iter > 0 && (file1_iter  == 0 || subsequence_comparison_grid[file1_iter ][file2_iter - 1] >= subsequence_comparison_grid[file1_iter  - 1][file2_iter]) {
        print_diff(subsequence_comparison_grid, file1, file2, file1_iter , file2_iter - 1);
        println!("> {}", file2[file2_iter - 1]);
    } else
    if file1_iter  > 0 && (file2_iter == 0 || subsequence_comparison_grid[file1_iter ][file2_iter - 1] < subsequence_comparison_grid[file1_iter  - 1][file2_iter]) {
        print_diff(subsequence_comparison_grid, file1, file2, file1_iter  - 1, file2_iter);
        println!("< {}", file1[file1_iter  - 1]);
    }
}
