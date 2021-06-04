pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: Vec<String> = vec![];

    if list.len() > 1 {
        for (i, word) in list.iter().enumerate() {
            if i < list.len() - 1 {
                proverb.push(format!("For want of a {} the {} was lost.", word, list[i + 1]));
            }
        }
    }

    if list.len() > 0 { proverb.push(format!("And all for the want of a {}.", list[0])); }

    proverb.join("\n")
}
