use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::*;
use itertools::{Either, Itertools};

const INPUT_FILE: &str = "data/two-lists.txt";

//region Version 1: Using loops
fn read_lists_loops<R: BufRead>(reader: R)
        -> Result<(Vec<usize>, Vec<usize>), Error> {
    let mut first = vec![];
    let mut second = vec![];
    for line in reader.lines() {
        let line = line?;
        //region inner_loop
        let mut numbers = [0, 0];
        let mut ix = 0;
        for c in line.chars() {
            //region processing `char`
            if let Some(digit) = c.to_digit(10) {
                numbers[ix] = numbers[ix] * 10 + digit as usize;
            } else if c.is_whitespace() {
                ix = 1;
            } else {
                return Err(anyhow!("Invalid input"));
            }
            //endregion
        }
        first.push(numbers[0]);
        second.push(numbers[1]);
        //endregion
    }
    Ok((first, second))
}
//endregion

//region Version 2: Functional style with iterators
fn parse_line_to_tuple(line: String) -> Result<(usize, usize), Error> {
    line
        .split_whitespace()
        .flat_map(str::parse)
        .collect_tuple()
        .ok_or_else(|| anyhow!("Invalid input"))
}

fn read_lists_fp<R: BufRead>(reader: R) -> Result<(Vec<usize>, Vec<usize>), Error> {
    Ok(reader
        .lines()
        .flatten()
        .map(parse_line_to_tuple)
        .collect::<Result<_>>()?)
}
//endregion

//region Version 3: Functional style with iterators, but differently
fn read_lists_3<R: BufRead>(mut reader: R) -> Result<(Vec<usize>, Vec<usize>), Error> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .enumerate()
        .partition_map(|(ix, e)| {
            if ix % 2 == 0 {
                Either::Left(e)
            } else {
                Either::Right(e)
            }
        }))
}
//endregion

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::*;


    const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_read_lists() {
        assert_eq!(read_lists_fp(BufReader::new(TEST.as_bytes())).unwrap(),
                   read_lists_loops(BufReader::new(TEST.as_bytes())).unwrap())
    }

    #[test]
    fn test_read_lists2() {
        assert_eq!(read_lists_fp(BufReader::new(TEST.as_bytes())).unwrap(),
                   read_lists_3(BufReader::new(TEST.as_bytes())).unwrap())
    }


    const TEST2: &str = "\
3   4
4   3
2   5a
1   3
3   9
3   3
";
    #[test]
    fn test_read_lists_err() {
        let res1 = read_lists_fp(BufReader::new(TEST2.as_bytes()));
        let res2 = read_lists_loops(BufReader::new(TEST2.as_bytes()));
        assert!(res1.is_err());
        assert!(res2.is_err());
        assert_eq!(res1.unwrap_err().to_string(), res2.unwrap_err().to_string());
    }
}

fn main() -> Result<()>{
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let (first, second) = read_lists_fp(input_file)?;
    println!("First list: len={}, sum={}", first.len(), first.iter().sum::<usize>());
    println!("Second list: len={}, sum={}", second.len(), second.iter().sum::<usize>());
    Ok(())
}