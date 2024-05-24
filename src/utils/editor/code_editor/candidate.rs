use crossterm::style::Stylize;

use crate::utils::loop_traverser::LoopTraverser;

pub struct Candidate {
    list: LoopTraverser<String>,
    word_width: usize,
}

impl Candidate {
    pub fn new() -> Self {
        Self {
            list: LoopTraverser::new(true),
            word_width: 0,
        }
    }

    pub fn set(&mut self, word: &str, candidates: Vec<String>) {
        let candidates = candidates
            .iter()
            .map(|item| word.to_string() + item)
            .collect();
        self.word_width = word.chars().count();
        self.list.set_content(candidates);
        self.next();
    }

    pub fn rendered_content(&self, line_width: usize) -> Option<String> {
        if self.list.is_empty() {
            return None;
        }

        let current_index = self.list.current_index()?;
        let is_selected_item = |index: usize| index == current_index;
        let selected_item_width = |item: &str| item.chars().count() + 3;
        let unselected_item_width = |item: &str| item.chars().count() + 1;
        let mut until_selected_width = 0;
        for (index, item) in self.list.iter().enumerate() {
            if is_selected_item(index) {
                until_selected_width += selected_item_width(item);
                break;
            } else {
                until_selected_width += unselected_item_width(item);
            }
        }

        let is_backward_render = until_selected_width > line_width;
        // rendering starts from `start_index`th item,
        // and the most left rendered item trucated width.
        let (start_index, truncated_width) = if !is_backward_render {
            (0_usize, 0_usize)
        } else {
            // highlighted item is at the right side of rendered bar,
            // and there is a overflow.
            let mut rendered_width = until_selected_width;
            let mut index = 0;
            for item in self.list.iter() {
                let item_len = unselected_item_width(item);
                if rendered_width - item_len <= line_width {
                    break;
                }
                index += 1;
                rendered_width -= item_len;
            }
            (index, rendered_width - line_width)
        };

        let mut remain_width = line_width;
        let mut result_content = String::new();
        let white_space = " ".on_grey().to_string();
        let selected_item_render = |item: &str| format!(" {} ", item).white().on_red();
        let unselected_item_render = |item: &str| (item.to_owned() + " ").black().on_grey();
        for (index, item) in self.list.iter().skip(start_index).enumerate() {
            if remain_width == 0 {
                break;
            }
            if index == 0 && truncated_width > 0 {
                let rendered_str = item[truncated_width..].to_owned() + " ";
                let rendered_width = rendered_str.chars().count();
                let rendered_styled = rendered_str.black().on_white();
                result_content += &rendered_styled.to_string();
                remain_width -= rendered_width;
                continue;
            }

            let is_selected = is_selected_item(index + start_index);
            let item_width = if is_selected {
                selected_item_width(item)
            } else {
                unselected_item_width(item)
            };

            if remain_width >= item_width {
                let item_rendered = if is_selected {
                    selected_item_render(item).to_string() + &white_space
                } else {
                    unselected_item_render(item).to_string()
                };
                result_content += &item_rendered;
                remain_width -= item_width;
            } else {
                let rendered_str = &item[..remain_width];
                let rendered_styled = rendered_str.black().on_white();
                result_content += &rendered_styled.to_string();
                remain_width = 0;
            }
        }

        if remain_width > 0 {
            let remain_white_spaces = white_space.repeat(remain_width);
            result_content += &remain_white_spaces;
        }
        return Some(result_content);
    }

    pub fn current(&self) -> Option<String> {
        let current_candidate = self.list.current()?;
        let actual_current = current_candidate.chars().skip(self.word_width).collect::<String>();
        return Some(actual_current);
    }
    #[inline]
    pub fn next(&mut self) -> Option<&String> {
        self.list.next()
    }
    #[inline]
    pub fn previous(&mut self) -> Option<&String> {
        self.list.previous()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.list.clear()
    }
}

#[test]
fn candidate_test() {
    const LINE_WIDTH: usize = 16;
    let mut candidates = Candidate::new();
    candidates.set(
        "ar",
        vec![
            String::from("g1"),
            String::from("g2"),
            String::from("g3"),
            String::from("g4"),
        ],
    );
    println!("{}", candidates.rendered_content(LINE_WIDTH).unwrap());
    candidates.next();
    println!("{}", candidates.rendered_content(LINE_WIDTH).unwrap());
    candidates.next();
    println!("{}", candidates.rendered_content(LINE_WIDTH).unwrap());
    candidates.next();
    println!("{}", candidates.rendered_content(LINE_WIDTH).unwrap());

    let mut candidates = Candidate::new();
    candidates.set("ar", vec![String::from("g1")]);
    println!("{}", candidates.rendered_content(LINE_WIDTH).unwrap());
}
