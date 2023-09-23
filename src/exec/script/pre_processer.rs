enum State {
    Indent,
    Code,
}

// remove indents and comments in code line
pub fn process(source: &str) -> String {
    let mut result = String::new();
    let mut state = State::Indent;

    for ch in source.chars() {
        if ch == '#' {
            // avoid comments
            break;
        }

        match state {
            State::Indent => {
                if !(ch == ' ' || ch == '\t') {
                    state = State::Code;
                    result.push(ch);
                    continue;
                }
            }
            State::Code => result.push(ch),
        }
    }

    // remove the WhiteSpaces and Tabs at the peak of result
    let mut white_space_count = 0;
    for ch in result.chars().rev() {
        if ch == ' ' || ch == '\t' {
            white_space_count += 1;
        } else {
            break;
        }
    }
    for _ in 0..white_space_count {
        result.pop();
    }

    return result;
}
