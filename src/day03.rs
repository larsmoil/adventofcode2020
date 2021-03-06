use std::vec::Vec;

pub fn pt1(input: Vec<String>) -> i32 {
    (0..input.len())
        .map(|row| {
            let terrain = &input[row];
            let point = terrain.chars().collect::<Vec<char>>()[row * 3 % terrain.len()];
            if point == '#' { 1 } else { 0 }
        })
        .fold(0, |a, b| a + b)
}

pub fn pt2(input: Vec<String>) -> i64 {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| {
            (0..(input.len() / down))
                .map(|step| {
                    let row = step * down;
                    let terrain = &input[row];
                    let point = terrain.chars().collect::<Vec<char>>()[step * right % terrain.len()];
                    if point == '#' { 1 } else { 0 }
                })
                .fold(0, |a, b| a + b)
        })
        .fold(1, |a, b| a * b)
}

pub fn input() -> Vec<String> {
    return vec![
        "......##....#...#..#.#....#....",
        ".......#...#..#..#....##.......",
        "#.#...#........###.#.##..#.....",
        ".......#.....##.#..##...##.##..",
        ".#...#.#...##..........#..#....",
        "#.......##.....###.#...#......#",
        ".........#....#.#.......#..#...",
        "..#.......####.......###..##...",
        ".#.#.##..#.#...##..#...###...##",
        "...................#...........",
        "....#.#.......#..........#.#..#",
        "..#.#...........####.#.......#.",
        ".....#.##..#..##..#.#...#......",
        "#.##...###..#................##",
        "...#...#...#..##.#............#",
        "#.##....##....#........#..#....",
        "..#......#.#.....##.......#....",
        ".......#......#....#......#....",
        ".#........##.....#.#...#...#.#.",
        "..........##.#...#..#..........",
        "#####..##......#.....#......#.#",
        "......#...............##...#...",
        "..#.#.##..#...#.#........#...#.",
        "..........#......#..........###",
        "..#...##.##..##..........#.....",
        "........#.##.#.....#..#...#....",
        "#.....#.........#..............",
        "..........##.##....#..#..#.....",
        "..#...........#.......#........",
        "........#..#.....#.#.#...#.....",
        "#.......##.....#.....#...#.##..",
        "###.#.#....#..#.....#........#.",
        "..#..#..#..........#....#....#.",
        "..#...##...#.#.##.....#..#.....",
        "...#....###...........##.#.....",
        ".##.................##.#.......",
        "........#...#.##..#...#........",
        ".##..#............##..........#",
        "............###.#....#..#......",
        ".....##....#.....#......#.....#",
        "....#.....#.##.......#...#.#...",
        ".##.#......#.........#...##....",
        "..##......#......#...........#.",
        ".......#.#.............#.......",
        ".##.#...#..##....##.......#....",
        "...#......##.#.#......#.....###",
        "#.#....#.......#.#......#....#.",
        "#......#.#.....#...........#..#",
        "##.#..##...#........#.##.#....#",
        ".....#........#........#...#...",
        "...............#.......#..#....",
        ".#.#.#..#.#...#.......#.....##.",
        ".#.#.............#..#....#.....",
        "....#.......#..##.........###..",
        ".#.....#.#....#..#..#....#.....",
        "........#......#.....#.#....#..",
        "##......#....##.....#.#..#.#...",
        ".#...#..#.##.#.##.##.....#.....",
        "#...#....#.........##.#....#...",
        ".........##..#.....#..#...#.#..",
        ".#............#..........#.#...",
        "...........#.....#......#.#....",
        "#...#...#.....#..#....#........",
        "#..##.....#..#.......#....#...#",
        "#..#..#..........#......#...#..",
        "...#...#.#.##.#...#....#...##..",
        "......##....##....#....##..####",
        "...###.#..#....#.......#.......",
        "#.........##......#...#........",
        "..........#....#.......#.......",
        "#....##................##....##",
        ".........#....#.#.......##.#...",
        ".....#......###.......#..#...##",
        "###.....#..##....###...........",
        ".....#...#....#.....##......###",
        ".#..#...#......##........##..#.",
        "#.#.#.#....#.............#.....",
        "......#.....##.#....#..##...#..",
        "..#............#.#....#..#...#.",
        ".............#.#...##.......#..",
        "...#....#.##.#...#.#..##...###.",
        "...#..............#.......#....",
        "......###.#............#.....#.",
        ".##...###..#.####...#..........",
        "...#..#...#.#.#..#......#..#...",
        ".#....##.###....#........#.....",
        "..#..#....#.........##.........",
        "..........##.###........#.#...#",
        ".........#...#..#........#.....",
        ".......#.....#...###...........",
        ".....#.#..##......#...#...#....",
        ".....#....#..#........##.#..#..",
        "...#...........#............#..",
        "##.....#....#.#...#...#....##..",
        "...#.....#.....#...##...#...#..",
        "...##.#..........##...#.#.##.#.",
        "....#.#.##.......#.#...#......#",
        "......###...#....#.##........#.",
        ".....#.........#...#...#..#..##",
        ".........#................#....",
        ".##..###..................#.#.#",
        ".##...........#...........#....",
        "#...#........#.....#..#...##...",
        ".....#..#...#.........#.......#",
        "..#..............#......#......",
        "#....#...............#.#.......",
        "...#........#.#....#..#.###.##.",
        ".......#..##..#...#..#...###...",
        "..........##..#.......##.##....",
        "##.#..#.#...##..........#......",
        ".#.##.#...##.....#....#....#.##",
        "...#.#......#...#.##..##.......",
        "##.......#.#......#....##..#.#.",
        "...#..#.##.........#...#.....#.",
        ".##.##..##...#........#..#.....",
        ".#.##.............#.#.#.....#..",
        ".......#.....................#.",
        "......#...#....#..#..........#.",
        "..#..#....#.#................#.",
        "..#.....#..#.#......#......###.",
        "...#...##..##....#..#...###.#..",
        "...#.....#............##......#",
        ".......#.#.#......#.....###....",
        ".....#......#.....#.........#.#",
        "#...#.#...#..#...#..#....#.....",
        "#..##...#..##.............#..#.",
        "##....##.......#.#.......#..#.#",
        "..............#...#..#......#..",
        "..#...#...#.#...#.#............",
        "#..........#...#.............#.",
        "..........##......#........#...",
        "#...#...#....#.#...........#...",
        "..#.#.#...##......#.#...#.#..#.",
        ".......#.......#.............#.",
        ".#..........#..................",
        "..##...#......#..........#....#",
        ".#..##..........#...#..........",
        "...#....#..#.#.....##..##.#..#.",
        "...#...#...#..#....##..#....#..",
        "..............#.#.....#......##",
        "..............####....#.#..#...",
        ".#........##....#...#.#...#..#.",
        ".#..##.###....#.#.....##..#....",
        "...###.#.........#..#..#.##.#..",
        ".....#..#.....#..#...##......##",
        ".#.#.##.............#...##.....",
        "....##........#........#.......",
        ".......#.....###..............#",
        "#.##.......##....#.#.....#.#...",
        "........#....#............#..##",
        "...#.#..#.......#..........#...",
        "..##....#..##......###.#.....#.",
        ".#..#.#.##....#.......#........",
        "........#.####.#.......#.##....",
        "..........##...............#...",
        ".#..#.....#....##..#..##...#..#",
        "....#.#.....#.#.........#####..",
        "...#.##....#...###.##.#..#.....",
        ".#...........#.............##.#",
        "..#....#....####.....#.#....#..",
        "......##.......#....#..#.......",
        ".####...##.#.#..#.####.#.#.....",
        "###.........#..#.#.#.#........#",
        "...#...#..#.............#.##...",
        ".........#....#......#.....#.#.",
        "...#....#......#..#......#....#",
        "..#...#..........##..##........",
        ".....##........#......#.....#..",
        "...#....#....#....#..#....#....",
        "##...#...........##............",
        ".......#..##..#.......##.#.....",
        "...............#.##.....#......",
        "#.#....##.#.....#...#..........",
        "........#......#...#......#.#.#",
        "..#..#.....#.#........#........",
        "..####.....##.#.##.......#.#.#.",
        ".#.##.#.......##......#.....#..",
        "....#.....##.........#.....#...",
        ".#.#...###.#.#..........#....#.",
        ".........##.#.#.....#..#.......",
        "......#..#...#..#..###.#.#.....",
        ".....#...#.#..#.#.......#.#...#",
        "......##........#..#...#......#",
        "#..##...#...#..#.....#..#..#..#",
        "......#....#...........#.#.....",
        "...#.......#...............#...",
        "#.........#......#.............",
        "..###..................#......#",
        "#.....#.#.#.......##....#......",
        ".........#...........#....#.#..",
        ".###....##.##..##.............#",
        ".##.#......#...#...##..........",
        "....#........###......#.#......",
        "...........#..#.##.#...........",
        ".#..#.......#......#.#####.....",
        "....##....##......#....#...#...",
        ".......#..#.....#.#...###...#.#",
        "..##.....#.......#.#.#..#.....#",
        ".#...#............#....##...#..",
        ".#..#...##.......#.............",
        "..##.......#...........#.#....#",
        "...#.#...#....#..#.....#.......",
        "...#........#...##...#.#..#.#..",
        "#........#..........#..........",
        "......#......#.........#.......",
        "...##...#.....#.......#...#.##.",
        "......##..##......#..###..#....",
        "....##....#..###.#.....##......",
        "##.##..#.....#..#..............",
        "..#.#..#....#....#....#.#...#.#",
        ".#.....##.#.##.#..#.#..........",
        "...#......##.#...##..##...#....",
        ".###.....#......#.......#.....#",
        "....##.......#.....#..#....#...",
        "..........#..##....#..##.#....#",
        "...#....#..##.#........#.#.#...",
        "...#.#...#....#.......#..##.#.#",
        "#..#..........#.#...#....#.....",
        "#..#...........................",
        "........#.....#.....#.#...#.#..",
        "#...#..#...#..........###...#.#",
        ".....##.#..##.#.#.#.##....#....",
        "#.......#....#.#..#..#..#.#....",
        "..###.#.......#.#.##...........",
        "#....#..#..........#.##..#.#...",
        "..#..#........##....#..##......",
        "#...##..#.........#.#....#.#...",
        "##..###..##...#.........#.#...#",
        "###..#....#..##...#.#..#.#.....",
        ".#.##.#......#............#....",
        ".#...#.##.#.........##.........",
        "##.....###.....#........#..#...",
        "...........##.#................",
        ".#......###......#....#..####..",
        "#...##.....#.....#..##....#.#..",
        "..#....#.......#.#.#......#...#",
        "#.....#........#....#.#...#....",
        "..##...............#....#..###.",
        ".#....#.......#..#...#.........",
        ".##.#..#..#...#..#..#....#....#",
        ".......#.#....#.....##...#.....",
        ".#....#.#.#...........#........",
        ".........#..##..#..#...#.......",
        "##..##...#......#.....#........",
        "#...........#.....#..###......#",
        ".#...........#....#...#...##.#.",
        "..............##.###.#.#####.##",
        "........#.#...#.............##.",
        "#...................###..#.##..",
        "#.....#...##...................",
        ".....##..........#..#.#........",
        ".#....##.#....#....###....#...#",
        ".......#.#...........#.#.....#.",
        "......#........###...#...##....",
        ".##..........#..#..#...........",
        "....#.......#..#.....##.#..#...",
        "..#.##......#..#.....#..#......",
        "......#...#..##....#.#..#..#.#.",
        "#.........................#...#",
        "###.#.......#......##....#..#..",
        "..##.###.#...#.............#...",
        ".....#...#...#......#....#####.",
        "#..........#.#.##.#.#.....#..#.",
        "....#.........#...#.#.........#",
        "#.##.........#...#...#.####..##",
        ".##.................#..........",
        "##.....#............#..#.#.....",
        "#.#...#.#........#........#...#",
        ".#...........#....#..#.......#.",
        ".#.......#..........##..#.##..#",
        ".#..##....#..##......#.#..##...",
        "#......#............#.......#.#",
        ".##...............#...#...#....",
        ".......##.#..#..##.....#.......",
        "...#.......#..###.....#....#...",
        "......#............#...........",
        "####............#.........#.##.",
        "#......#.#..#...#.....#..#.....",
        "...........#...#..##.......####",
        "#.#...##..#....#.#.........#.#.",
        "...#....#..#.......#.........#.",
        ".........#.#.#...#....#........",
        ".#.....#........#..#.........#.",
        "....#....#..#.....#...#........",
        "..#....#.#.....#..##...........",
        ".#...#..#..#.##.###....#.......",
        "#......##.......##..##.........",
        "...#.........#.......##.#......",
        ".#...#...#.......#........##...",
        "..#.............#.......#.....#",
        "..#...........#.#.#...#.......#",
        ".....##..#....#..............#.",
        "#.#.....#.#....................",
        ".....#..##..#...#.....#........",
        "..#.......#..####..#....#.##.#.",
        "#....#.....#.....#...#......#..",
        "..#....##...#....#..#..#.....#.",
        "..#.####..............##.......",
        ".#.........#..#...#.......##...",
        "#....#.#........#....#...#...##",
        ".....#..#....#.#..#...#.#.##...",
        ".##.................#...##.....",
        ".##.##.##...#...........#...##.",
        "..#....#..#.....#..#......##...",
        ".#...........#......#....#..#.#",
        ".#.#............#..#..#...#....",
        "....#......#.....#.#.#.....#...",
        "#.......##.............#.......",
        "....#....................#.#...",
        "......#........#..#.#.....#.#..",
        ".....#..#....#.#........#....#.",
        "...##.........#...#.##....#..#.",
        ".#....#..#...#.#.#......#......",
        "#......#.#.##.#..#..#.....##...",
        "......#....#.#...#..#.#........",
        "..#.....##.....#...#.#.......#.",
        "......#.#.....#........#.......",
        "......#.#.#...#..#.#.#.#.......",
        "..#.#.##..#..#..#.#.##...#.....",
        "......#.#.#......#.....#...#...",
        ".....#.##....#..##...#...#....#",
        "..#.....#...........#..#..##...",
        "..#..#.......#....#....###.#..."
    ].iter().map(|s| s.to_string()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        return vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ].iter().map(|s| s.to_string()).collect();
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(pt1(example_input()), 7);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(input()), 252);
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(pt2(example_input()), 336);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(pt2(input()), 2608962048);
    }
}
