const SPACE_ASCII      : u8 = 32; // ' '
const TAB_ASCII        : u8 = 9;  // '\t'
const NUMBER_SIGN_ASCII: u8 = 35; // #

enum State {
    Indent,
    Code,
}

pub fn process(source: String) -> String {
    let chars = source.as_bytes();
    let mut result = String::new();
    let mut index = 0;

    let mut state = State::Indent;

    while index < chars.len() {
        let current = chars[index];

        match state {
            State::Indent => {
                if !(current == SPACE_ASCII || current == TAB_ASCII) {
                    state = State::Code;
                    continue;
                }
            },
            State::Code => {
                if current == NUMBER_SIGN_ASCII { break }
                result.push(current as char)
            }
        }

        index += 1;
    }
    return result
}