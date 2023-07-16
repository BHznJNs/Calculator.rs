enum State {
    Indent,
    Code,
}

pub fn process(source: String) -> String {
    let mut chars = source.chars();
    let mut result = String::new();

    let mut state = State::Indent;

    while let Some(ch) = chars.next() {
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
    return result;
}
