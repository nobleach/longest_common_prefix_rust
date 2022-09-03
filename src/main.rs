use longest_common_prefix::find_longest_common_prefix;

fn main() {
    let strs = vec!["flower", "flow", "flight", "fly", "floor", "flu", "flag"];
    let res = find_longest_common_prefix(strs);

    println!("{}", res);
}
