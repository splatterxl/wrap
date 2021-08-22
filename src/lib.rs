pub const LIMIT: usize = 80;

pub fn wrap(text: String) -> String {
    let lines = text.split(|c| c == '\n');

    let mut to_return = "".to_owned();

    for line in lines {
        let words = line.split(|c| c == ' ');

        let mut new_line = "".to_owned();

        let mut size: usize = 0;

        for word in words.clone() {
            if word.is_empty() {
                new_line = format!("{} ", new_line);
                continue;
            }
            let len = word.len();

            size += &len;

            if size <= LIMIT {
                let space = {
                    if new_line.is_empty() {
                        ""
                    } else {
                        " "
                    }
                };
                new_line = format!("{}{}{}", new_line, space, word);
                continue;
            } else {
                new_line = format!("{}\n{}", new_line, word);
                size = len;
                continue;
            }
        }

        to_return = format!("{}\n{}", to_return, new_line);
    }

    String::from(to_return.trim())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    pub fn does_it_work() {
        let gnu = fs::read_to_string("test/test_case.txt").expect("Did not find test_case.txt file");
        let to_assert = fs::read_to_string("test/wrapped.txt").expect("Did not find wrapped.txt file");
        assert_eq!(crate::wrap(gnu), to_assert.trim_end());
    }
}
