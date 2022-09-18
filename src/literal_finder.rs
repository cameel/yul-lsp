use yultsur::visitor::ASTVisitor;
use yultsur::yul::{Block, Literal, SourceLocation};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
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

    pub fn between(&self, location: &SourceLocation) -> bool {
        location.start <= self.cursor_location && self.cursor_location < location.end
    }
}

impl ASTVisitor for LiteralFinder {
    fn visit_literal(&mut self, literal: &Literal) {
        if let Some(location) = &literal.location {
            // TODO: More strict heuristics
            if self.between(location)
                && self.literal_kind == LiteralKind::Selector
                && literal.literal.len() == 10
                || self.literal_kind == LiteralKind::Address && literal.literal.len() == 42
            {
                assert!(self.found_literal.is_none());
                self.found_literal = Some(literal.clone());
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
    literal_finder.visit_block(ast);
    literal_finder.found_literal
}
