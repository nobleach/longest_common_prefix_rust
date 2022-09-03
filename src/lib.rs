pub fn find_longest_common_prefix(mut list: Vec<&str>) -> String {
    list.sort();
    let first_str = list.first();
    let last_str = list.last();
    let mut pointer = 0;
    let mut longest_prefix = String::from("");

    while pointer <= first_str.unwrap().len() -1 {
       if first_str.unwrap().chars().nth(pointer).unwrap() != last_str.unwrap().chars().nth(pointer).unwrap() {
           break;
       }

       longest_prefix.push(first_str.unwrap().chars().nth(pointer).unwrap());
       pointer += 1;
    }

    return longest_prefix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_fl() {
        let strs = vec!["flower", "flow", "flight", "fly", "floor", "flu", "flag"];
        let result = find_longest_common_prefix(strs);

        assert_eq!(result, "fl");
    }

    #[test]
    fn it_finds_nothing() {
        let strs = vec!["flower", "glow", "flight", "fly", "floor", "flu", "flag"];
        let result = find_longest_common_prefix(strs);

        assert_eq!(result, "");
    }
}

