use yultsur::visitor::ASTVisitor;
use yultsur::yul::{Identifier, SourceLocation};
use yultsur::yul_parser::parse_block;

struct IdentifierFinder {
    pub cursor_location: usize,
    pub found_identifier: Option<Identifier>,
}

impl IdentifierFinder {
    pub fn new(cursor_location: usize) -> IdentifierFinder {
        IdentifierFinder {
            cursor_location,
            found_identifier: None,
        }
    }
}

impl ASTVisitor for IdentifierFinder {
    fn visit_identifier(&mut self, identifier: &Identifier) {
        if let Some(location) = &identifier.location {
            if location.start <= self.cursor_location && self.cursor_location < location.end {
                // Identifiers cannot overlap
                assert!(self.found_identifier == None);

                self.found_identifier = Some(identifier.clone())
            }
        }
    }
}

pub fn find_identifier(
    source_code: &str,
    cursor_position: usize,
) -> Result<Option<Identifier>, String> {
    let ast = parse_block(&source_code)?;

    let mut identifier_finder = IdentifierFinder::new(cursor_position);
    identifier_finder.visit_block(&ast);
    Ok(identifier_finder.found_identifier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use std::matches;

    #[test]
    fn erc20_identifier_not_found() {
        let source_code = read_to_string("examples/erc20.yul").unwrap();
        let result = find_identifier(&source_code, 0);

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn erc20_require() {
        let source_code = read_to_string("examples/erc20.yul").unwrap();
        let result = find_identifier(&source_code, 10);

        assert!(matches!(result, Ok(Some(_))));
        let Identifier {
            id: _,
            name,
            location,
        } = result.unwrap().unwrap();
        assert_eq!(name, "require");
        assert!(matches!(location, Some(_)));
        assert_eq!(location.unwrap(), SourceLocation { start: 6, end: 13 });
    }
}
