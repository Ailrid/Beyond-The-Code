use comrak::nodes::{ Node};
use comrak::{Arena, Options, parse_document};

pub struct Analyzer<'a> {
    pub options: Options<'static>,
    pub arena: Arena<'a>,
}

impl<'a> Analyzer<'a> {
    pub fn new() -> Self {
        let mut options = Options::default();
        options.extension.math_dollars = true;
        options.extension.cjk_friendly_emphasis = true;
        options.extension.table = true;
        Analyzer {
            options: options,
            arena: Arena::new(),
        }
    }
    pub fn parse<'b: 'a>(&'b self, markdown: &str) -> Node<'b> {
        let node = parse_document(&self.arena, markdown, &self.options);
        node
    }
    pub fn reset(&mut self) {
        self.arena = Arena::new();
    }
}
