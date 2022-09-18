use yultsur::visitor::ASTVisitor;
use yultsur::yul::{Block, Literal};

#[derive(Hash, Clone, PartialEq, Debug)]
pub enum LiteralKind {
    Selector,
    Address,
}
struct LiteralFinder {
    pub cursor_location: usize,
    pub literal_kind: LiteralKind,
    pub found_literal: Option<Literal>,
}

impl LiteralFinder {
    pub fn new(cursor_location: usize, literal_kind: LiteralKind) -> LiteralFinder {
        LiteralFinder {
            cursor_location,
            literal_kind,
            found_literal: None,
        }
    }
}

impl ASTVisitor for LiteralFinder {
    fn visit_literal(&mut self, literal: &Literal) {
        if let Some(location) = &literal.location {
            if location.start <= self.cursor_location && self.cursor_location < location.end {
                if
                // TODO: More strict heuristics
                self.literal_kind == LiteralKind::Selector && literal.literal.len() == 10
                    || self.literal_kind == LiteralKind::Address && literal.literal.len() == 42
                {
                    assert!(self.found_literal == None);
                    self.found_literal = Some(literal.clone());
                }
            }
        }
    }
}

pub fn find_literal(
    ast: &Block,
    cursor_position: usize,
    literal_kind: LiteralKind,
) -> Option<Literal> {
    let mut literal_finder = LiteralFinder::new(cursor_position, literal_kind);
    literal_finder.visit_block(&ast);
    literal_finder.found_literal
}
