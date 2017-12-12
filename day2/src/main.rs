fn main() {
    // Read file into a string
    let contents = include_str!("../data.txt");

    let num_array = parse(&contents);

    let checksum_part1: i32 = num_array.iter().map(line_diff).sum();
    let checksum_part2: i32 = num_array.iter().map(line_even_divide_sum).sum();
    println!("Part 1 : {}", checksum_part1);
    println!("Part 2 : {}", checksum_part2);
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    s.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().flat_map(str::parse).collect()
}

fn line_diff(line: &Vec<i32>) -> i32 {
    (line.iter().max().unwrap() - line.iter().min().unwrap()).abs()
}

//TODO Refactor ugly for loops
fn line_even_divide_sum(line: &Vec<i32>) -> i32 {
    for value1 in line {
        for value2 in line {
            if (value1 != value2) & (value1 % value2 == 0) {
                return value1 / value2
            }
        }
    }
    panic!("Didn't find even values in line")
}

#[cfg(test)]
mod tests {
    use parse;
    use line_diff;
    use line_even_divide_sum;

    #[test]
    fn test_p1() {
        let contents = include_str!("../datatest1.txt");

        let num_array = parse(&contents);

        let checksum: i32 = num_array.iter().map(line_diff).sum();
        assert_eq!(checksum, 18);
    }

    #[test]
    fn test_p2() {
        let contents = include_str!("../datatest2.txt");

        let num_array = parse(&contents);

        let checksum: i32 = num_array.iter().map(line_even_divide_sum).sum();
        assert_eq!(checksum, 9);
    }

}