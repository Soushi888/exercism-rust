pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for (i, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (j, ch) in row.chars().enumerate() {
            match ch {
                '*' => new_row.push('*'),
                _ => {
                    let mut count = 0;
                    for x in i.saturating_sub(1)..=i + 1 {
                        for y in j.saturating_sub(1)..=j + 1 {
                            if x < minefield.len() && y < row.len() {
                                if minefield[x].chars().nth(y).unwrap() == '*' {
                                    count += 1;
                                }
                            }
                        }
                    }
                    if count > 0 {
                        new_row.push_str(&count.to_string());
                    } else {
                        new_row.push(' ');
                    }
                }
            }

        }
        result.push(new_row);
    }

    result
}
