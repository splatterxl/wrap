pub const LIMIT: usize = 80;

pub fn wrap(text: String) -> String {
    let lines = text.split(|c| c == '\n');

    let mut to_return = "".to_owned();

    for line in lines {
        let words = line.split(|c| c == ' ');

        let mut new_line = "".to_owned();

        let mut size: usize = 0;

        for word in words.clone() {
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
        let gnu = fs::read_to_string("gnu.txt").expect("Did not find gnu.txt file in CWD");
        assert_eq!(crate::wrap(gnu), "What you're refering to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it,\n\
GNU plus Linux. Linux is not an operating system unto itself, but rather another free component\n\
of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital\n\
system components comprising a full OS as defined by POSIX.\n\
\n\
Many computer users run a modified version of the GNU system every day, without realizing it.\n\
Through a peculiar turn of events, the version of GNU which is widely used today is often called\n\
Linux, and many of its users are not aware that it is basically the GNU system, developed by the\n\
GNU Project.\n\
\n\
There really is a Linux, and these people are using it, but it is just a part of the system they use.\n\
Linux is the kernel: the program in the system that allocates the machine's resources to the\n\
other programs that you run. The kernel is an essential part of an operating system, but useless\n\
by itself; it can only function in the context of a complete operating system. Linux is normally\n\
used in combination with the GNU operating system: the whole system is basically GNU with Linux\n\
added, or GNU/Linux. All the so-called Linux distributions are really distributions of\n\
GNU/Linux!")
    }
}
