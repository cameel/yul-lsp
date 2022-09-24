use crate::identifier_finder::find_identifier;
use yultsur::dialect::EVMDialect;
use yultsur::resolver::resolve;
use yultsur::visitor::ASTVisitor;
use yultsur::yul::{Identifier, IdentifierID};
use yultsur::yul_parser::parse_block;

struct DefinitionFinder {
    pub reference_id: u64,
    pub found_identifier: Option<Identifier>,
}

impl DefinitionFinder {
    pub fn new(reference_id: u64) -> DefinitionFinder {
        DefinitionFinder {
            reference_id,
            found_identifier: None,
        }
    }
}

impl ASTVisitor for DefinitionFinder {
    fn visit_identifier(&mut self, identifier: &Identifier) {
        if let IdentifierID::Declaration(id) = identifier.id {
            if id == self.reference_id {
                // Identifier can have only one definition
                assert!(self.found_identifier.is_none());

                self.found_identifier = Some(identifier.clone())
            }
        }
    }
}

pub fn find_definition(
    source_code: &str,
    cursor_position: usize,
) -> Result<Option<Identifier>, String> {
    let mut ast = parse_block(source_code)?;
    resolve::<EVMDialect>(&mut ast);

    if let Some(reference) = find_identifier(&ast, cursor_position) {
        match reference.id {
            IdentifierID::Declaration(_) => Ok(Some(reference)),
            IdentifierID::Reference(id) => {
                let mut definition_finder = DefinitionFinder::new(id);
                definition_finder.visit_block(&ast);
                if let Some(definition) = &definition_finder.found_identifier {
                    assert!(matches!(definition.id, IdentifierID::Declaration(_)));
                }
                Ok(definition_finder.found_identifier)
            }
            IdentifierID::BuiltinReference => Ok(None),
            IdentifierID::UnresolvedReference => Ok(None),
        }
    } else {
        Ok(None)
    }
}
