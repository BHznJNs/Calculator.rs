use crate::public::env::ENV;

pub struct CodeLine {
    pub indent: usize,
    pub comment: String,
    pub content: String,
}

// do pre-process jobs for inputted code line
// and returns `CodeLine`
pub fn process(source: &str) -> CodeLine {
    enum State {
        Indent,
        Code,
        Comment,
    }

    let mut state = State::Indent;

    let set_indent = unsafe { ENV.options.indent_size };
    let mut temp_indent = 0;
    let mut result_indent = 0;

    let mut result_content = String::new();
    let mut result_comment = String::new();

    for ch in source.chars() {
        if ch == '#' {
            state = State::Comment;
            continue;
        }

        match state {
            State::Indent => {
                if ch == ' ' {
                    temp_indent += 1;
                    if temp_indent == set_indent {
                        result_indent += 1;
                        temp_indent = 0;
                    }
                } else if ch == '\t' {
                    result_indent += 1;
                    temp_indent = 0;
                } else {
                    result_content.push(ch);
                    state = State::Code;
                }
            },
            State::Code => result_content.push(ch),
            State::Comment => result_comment.push(ch),
        }
    }
    return CodeLine {
        indent: result_indent,
        comment: result_comment,
        content: result_content,
    };
}

#[test]
fn pre_processor_test() {
    let test_content = "  codes # comment";
    let processed = self::process(test_content);
    assert_eq!(processed.indent, 1);
    assert_eq!(processed.comment, String::from(" comment"));
    assert_eq!(processed.content, String::from("codes "));

    // --- --- --- --- --- ---

    let test_content = "codes #";
    let processed = self::process(test_content);
    assert_eq!(processed.indent, 0);
    assert_eq!(processed.comment, String::from(""));
    assert_eq!(processed.content, String::from("codes "));

    // --- --- --- --- --- ---

    let test_content = "     codes";
    let processed = self::process(test_content);
    assert_eq!(processed.indent, 2);
    assert_eq!(processed.comment, String::from(""));
    assert_eq!(processed.content, String::from("codes"));
}