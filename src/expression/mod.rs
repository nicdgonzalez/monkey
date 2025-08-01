mod boolean;
mod call;
mod function_literal;
mod grouped;
mod identifier;
mod r#if;
mod infix;
mod integer_literal;
mod prefix;

use crate::evaluator::Evaluate;
use crate::object::NULL;
use crate::parser::{INFIX, PREFIX, Parser, ParserError};
use crate::precedence::{PRECEDENCES, Precedence};
use crate::token::TokenKind;

pub use boolean::Boolean;
pub use call::Call;
pub use function_literal::FunctionLiteral;
pub use grouped::Grouped;
pub use identifier::Identifier;
pub use r#if::If;
pub use infix::Infix;
pub use integer_literal::IntegerLiteral;
pub use prefix::Prefix;

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
    Infix(Infix),
    Boolean(Boolean),
    If(If),
    FunctionLiteral(FunctionLiteral),
    Call(Call),
}

impl Expression {
    pub fn parse(parser: &mut Parser<'_>, precedence: Precedence) -> Result<Self, ParserError> {
        assert_ne!(parser.token(), None, "Expression::parse called after EOF");
        let token = parser.token().unwrap().to_owned();
        tracing::debug!("{token:?}");
        let mut left = PREFIX
            .get(&token.kind())
            .map(|callback| callback(parser)) // NOTE: `callback` advances the parser.
            .expect(&format!("missing parse_prefix fn: {:?}", token))?;

        let span = tracing::trace_span!("while");
        while parser
            .token()
            .is_some_and(|token| token.kind() != TokenKind::Semicolon)
        {
            let _enter = span.enter();

            let token = parser.token().unwrap();
            tracing::debug!("{token:?}");
            let peek_precedence = PRECEDENCES
                .get(&token.kind())
                .unwrap_or(&Precedence::Lowest);

            tracing::debug!(
                "{:?} < {:?} (breaking = {})",
                *peek_precedence,
                precedence,
                *peek_precedence < precedence
            );

            if *peek_precedence < precedence {
                break;
            }

            if let Some(&infix) = INFIX.get(&token.kind()) {
                left = infix(parser, left)?;
            } else {
                break;
            }
        }

        Ok(left)
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, env: &mut crate::environment::Environment) -> crate::object::Object {
        let inner: &dyn Evaluate = match *self {
            Self::Identifier(ref inner) => inner,
            Self::IntegerLiteral(ref inner) => inner,
            Self::Prefix(ref inner) => inner,
            Self::Infix(ref inner) => inner,
            Self::Boolean(ref inner) => inner,
            Self::If(ref inner) => inner,
            Self::Call(_) | Self::FunctionLiteral(_) => return NULL,
        };

        (*inner).evaluate(env)
    }
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

impl From<IntegerLiteral> for Expression {
    fn from(value: IntegerLiteral) -> Self {
        Self::IntegerLiteral(value)
    }
}

impl From<Prefix> for Expression {
    fn from(value: Prefix) -> Self {
        Self::Prefix(value)
    }
}

impl From<Infix> for Expression {
    fn from(value: Infix) -> Self {
        Self::Infix(value)
    }
}

impl From<Boolean> for Expression {
    fn from(value: Boolean) -> Self {
        Self::Boolean(value)
    }
}

impl From<If> for Expression {
    fn from(value: If) -> Self {
        Self::If(value)
    }
}

impl From<FunctionLiteral> for Expression {
    fn from(value: FunctionLiteral) -> Self {
        Self::FunctionLiteral(value)
    }
}

impl From<Call> for Expression {
    fn from(value: Call) -> Self {
        Self::Call(value)
    }
}
