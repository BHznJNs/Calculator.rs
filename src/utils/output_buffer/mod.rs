use crate::utils::terminal::Terminal;

pub struct OutputBuffer {
    pub(self) print_buffer: String,
    pub(self) error_buffer: String,
}

static mut STATIC_OUTPUT_BUFFER: Option<OutputBuffer> = None;

impl OutputBuffer {
    pub fn init() {
        unsafe {
            STATIC_OUTPUT_BUFFER = Some(OutputBuffer {
                print_buffer: String::new(),
                error_buffer: String::new(),
            })
        };
    }

    #[inline]
    fn get() -> &'static mut OutputBuffer {
        unsafe { STATIC_OUTPUT_BUFFER.as_mut().unwrap() }
    }

    pub fn print_append(str: &str, flush: bool) {
        let buf = &mut Self::get().print_buffer;
        buf.push_str(str);
        if flush {
            Self::flush_to_terminal(buf);
        }
    }

    pub fn error_append(str: &str, flush: bool) {
        let buf = &mut Self::get().error_buffer;
        buf.push_str(str);
        if flush {
            Self::flush_to_terminal(buf);
        }
    }

    // print contents in buffer to terminal
    pub fn flush_to_terminal(buf: &mut String) {
        print!("{}\r\n", buf);
        Terminal::flush().expect("IO Error!");
        buf.clear();
    }

    pub fn clear() {
        let static_buffer = Self::get();
        static_buffer.print_buffer.clear();
        static_buffer.error_buffer.clear();
    }
}
