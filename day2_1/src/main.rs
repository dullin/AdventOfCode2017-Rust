fn main() {
    // Read file into a string
    let contents = include_str!("../data.txt");

    let num_array = parse(&contents);

    let checksum: i32 = num_array.iter().map(line_diff).sum();
    println!("{}", checksum);
}

fn line_diff(line: &Vec<i32>) -> i32 {
    (*line.iter().max().unwrap() - *line.iter().min().unwrap()).abs()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().flat_map(str::parse).collect()
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    s.lines().map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use parse;
    use line_diff;

    #[test]
    fn test_1() {
        let mut file = File::open("datatest.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let num_array = parse(&contents);

        let checksum: i32 = num_array.iter().map(line_diff).sum();
        assert_eq!(checksum, 18);
    }

}