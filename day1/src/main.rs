fn main() {
    // Read file into a string
    let contents = include_str!("../data.txt");

    //Convert string into array of digits
    let num_array:Vec<u32> = contents.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect();

    println!("Part 1 : {}", sum_next_array(&num_array));
    println!("Part 2 : {}", sum_mid_array(&num_array));
}

fn sum_next_array(num_array: &Vec<u32>) -> u32{

    //Check for consequent digits
    //Note : last returns an address
    let mut last = num_array.last().unwrap();
    let mut sum = 0;
    for i in num_array {
        if i == last {
            sum += i;
        }
        last = i;
    }
    sum
}

fn sum_mid_array(num_array : &Vec<u32>) -> u32{
    
    //Watch the call order, skip last or cycle over splitted list
    let mut fwd5 = num_array.iter().cycle().skip(num_array.len()/2);
    //Check for consequent digits
    //Note : last returns an address
    let mut sum = 0;
    for i in num_array {
        if i == fwd5.next().unwrap() {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use sum_next_array;
    use sum_mid_array;

    #[test]
    fn test_p1_1() {
        let num_vec = vec![1, 1, 2, 2];
        assert_eq!(sum_next_array(&num_vec), 3);
    }

    #[test]
    fn test_p1_2() {
        let num_vec = vec![1, 1, 1, 1];
        assert_eq!(sum_next_array(&num_vec), 4);
    }

    #[test]
    fn test_p1_3() {
        let num_vec = vec![1, 2, 3, 4];
        assert_eq!(sum_next_array(&num_vec), 0);
    }

    #[test]
    fn test_p1_4() {
        let num_vec = vec![9,1,2,1,2,1,2,9];
        assert_eq!(sum_next_array(&num_vec), 9);
    }

    #[test]
    fn test_p2_1() {
        let num_vec = vec![1, 2, 1, 2];
        assert_eq!(sum_mid_array(&num_vec), 6);
    }

    #[test]
    fn test_p2_2() {
        let num_vec = vec![1, 2, 2, 1];
        assert_eq!(sum_mid_array(&num_vec), 0);
    }

    #[test]
    fn test_p2_3() {
        let num_vec = vec![1, 2, 3, 4, 2 ,5];
        assert_eq!(sum_mid_array(&num_vec), 4);
    }

    #[test]
    fn test_p2_4() {
        let num_vec = vec![1,2,3,1,2,3];
        assert_eq!(sum_mid_array(&num_vec), 12);
    }

    #[test]
    fn test_p2_5() {
        let num_vec = vec![1,2,1,3,1,4,1,5];
        assert_eq!(sum_mid_array(&num_vec), 4);
    }
}