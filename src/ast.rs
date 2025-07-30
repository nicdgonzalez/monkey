use crate::expression::Expression;
use crate::program::Program;
use crate::statement::Statement;

#[derive(Debug)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl From<Program> for Node {
    fn from(value: Program) -> Self {
        Self::Program(value)
    }
}

impl From<Statement> for Node {
    fn from(value: Statement) -> Self {
        Self::Statement(value)
    }
}

impl From<Expression> for Node {
    fn from(value: Expression) -> Self {
        Self::Expression(value)
    }
}

impl Node {
    /// Returns the inner [`Program`], or `None` if the variant type is not [`Node::Program`].
    pub fn into_program(self) -> Option<Program> {
        match self {
            Self::Program(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the inner [`Program`] stored within the [`Node`].
    ///
    /// # Panics
    ///
    /// This function panics if the variant type is not [`Node::Program`].
    pub fn into_program_unchecked(self) -> Program {
        match self {
            Self::Program(inner) => inner,
            _ => panic!("expected Program variant"),
        }
    }

    /// Returns the inner [`Statement`], or `None` if the variant type is not [`Node::Statement`].
    pub fn into_statement(self) -> Option<Statement> {
        match self {
            Self::Statement(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the inner [`Statement`] stored within the [`Node`].
    ///
    /// # Panics
    ///
    /// This function panics if the variant type is not [`Node::Statement`].
    pub fn into_statement_unchecked(self) -> Statement {
        match self {
            Self::Statement(inner) => inner,
            _ => panic!("expected Statement variant"),
        }
    }

    /// Returns the inner [`Expression`], or `None` if the variant type is not [`Node::Expression`].
    pub fn into_expression(self) -> Option<Expression> {
        match self {
            Self::Expression(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the inner [`Expression`] stored within the [`Node`].
    ///
    /// # Panics
    ///
    /// This function panics if the variant type is not [`Node::Expression`].
    pub fn into_expression_unchecked(self) -> Expression {
        match self {
            Self::Expression(inner) => inner,
            _ => panic!("expected Expression variant"),
        }
    }
}
