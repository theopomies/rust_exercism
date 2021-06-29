pub fn build_proverb(list: &[&str]) -> String {
    (0..list.len())
        .map(|idx| {
            if idx == list.len() - 1 {
                return format!("And all for the want of a {}.", list[0]);
            }
            format!(
                "For want of a {} the {} was lost.",
                list[idx],
                list[idx + 1]
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
