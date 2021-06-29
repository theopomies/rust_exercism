pub fn raindrops(n: u32) -> String {
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => "PlingPlangPlong",
        (_, 0, 0) => "PlangPlong",
        (0, _, 0) => "PlingPlong",
        (0, 0, _) => "PlingPlang",
        (_, _, 0) => "Plong",
        (_, 0, _) => "Plang",
        (0, _, _) => "Pling",
        _ => return n.to_string(),
    }
    .to_owned()
}
