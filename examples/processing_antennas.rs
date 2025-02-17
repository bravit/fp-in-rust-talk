use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn to_signed((a, b): (usize, usize)) -> (isize, isize) {
    (a as isize, b as isize)
}

fn within_map((r, c): (isize, isize), width: usize, height: usize) -> Option<(usize, usize)> {
    if r < 0 || r >= height as isize || c < 0 || c >= width as isize {
        None
    } else {
        Some((r as usize, c as usize))
    }
}

fn find_antinodes(
        width: usize, height: usize,
        antennas: &HashMap<char, Vec<(usize, usize)>>)
            -> HashSet<(usize, usize)> {
    antennas.iter().flat_map(|(_, coords)|
        coords.iter().combinations(2).flat_map(|ants| {
            let (r1, c1) = to_signed(*ants[0]);
            let (r2, c2) = to_signed(*ants[1]);
            let dr = r2 - r1; let dc = c2 - c1;
            (0..).map(|i|
                [1, -1].map(|sign| {
                    let antinode = (r1 + dr * sign * i, c1 + dc * sign * i);
                    within_map(antinode, width, height)
                }))
                .take_while(|&x| x != [None,None])
                .flatten()
                .flatten()
                .collect::<Vec<_>>()
        })
    ).collect()
}

fn main() {
    let _set = find_antinodes(10, 10, &HashMap::new());
}