pub fn parse(s: String) -> Vec<isize> {
    s.split(",")
        .map(|i| i.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}
