/// The default limit for line size.
///
/// This can be changed via the [`limit`] parameter in [wrap()].
///
/// [wrap()]: fn.wrap.html
/// [`limit`]: fn.wrap.html#custom-limit
pub const LIMIT: usize = 80;

/// Wrap a `String`'s lines when they go over the specified `limit` (default `80`).
///
/// # Usage
///
/// A common usage of this function would go something like this:
///
/// ```
/// let file = std::fs::read_to_string("file.txt").unwrap();
/// // this is just for the example, don't do it in your actual code!
/// // you could use .unwrap_or() instead.
/// println!("{}", wrap(file, None));
///
/// ```
///
/// That code isn't safe, though. There's a [folder full of examples][examples] in the [GitHub repository][repo] which you can explore (and maybe take some *inspiration* from)!
///
/// # Custom Limit
///
/// It is possible to set a custom line width limit with the `limit` argument. In the binary executable bundled with the crate, it queries the `WRAP_LINE_SIZE` environment variable, using code similar to this:
///
/// ```
/// let limit = env!("WRAP_LINE_SIZE");
///
/// wrap(text, Some(limit))
/// ```
///
/// Of course, you can provide hard-coded values as well (although it isn't recommended!), and any
/// `usize` is accepted.
///
/// Ergo, `30usize` is valid, as well as `1usize`! Although, why would you even try to set the limit to `1`?!
///
/// [examples]: https://github.com/nearlySplat/wrap/tree/trunk/examples
/// [repo]: https://github.com/nearlySplat/wrap
pub fn wrap(text: String, limit: Option<usize>) -> String {
    let limit = limit.unwrap_or(LIMIT);
    let lines = text.split('\n');

    let mut to_return = "".to_owned();

    for line in lines {
        let words = line.split(' ');

        let mut new_line = "".to_owned();

        let mut size: usize = 0;

        for word in words.clone() {
            if word.is_empty() {
                new_line = format!("{} ", new_line);
                continue;
            }
            let len = word.len();

            size += &len;

            if size <= limit {
                let space = {
                    if new_line.is_empty() {
                        ""
                    } else {
                        " "
                    }
                };
                new_line = format!("{}{}{}", new_line, space, word);
            } else {
                new_line = format!("{}\n{}", new_line, word);
                size = len;
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
        let gnu =
            fs::read_to_string("test/test_case.txt").expect("Did not find test_case.txt file");
        let to_assert =
            fs::read_to_string("test/wrapped.txt").expect("Did not find wrapped.txt file");
        assert_eq!(crate::wrap(gnu, None), to_assert.trim_end());
    }
}
