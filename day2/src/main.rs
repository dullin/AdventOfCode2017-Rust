fn main() {
    // Read file into a string
    let contents = include_str!("../data.txt");

    let num_array = parse(&contents);

    let checksumPart1: i32 = num_array.iter().map(line_diff).sum();
    println!("Part 1 : {}", checksumPart1);
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    s.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().flat_map(str::parse).collect()
}

fn line_diff(line: &Vec<i32>) -> i32 {
    (*line.iter().max().unwrap() - *line.iter().min().unwrap()).abs()
}

fn line_even_divide_sum(line: &Vec<i32>) -> i32 {
    for value1 in line {
        even_div = 0;
        for value2 in line {
            if value1 != value2 & value1 % value2 == 0 {
                
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use parse;
    use line_diff;

    #[test]
    fn test_p1() {
        let contents = include_str!("../datatest.txt");

        let num_array = parse(&contents);

        let checksum: i32 = num_array.iter().map(line_diff).sum();
        assert_eq!(checksum, 18);
    }

}