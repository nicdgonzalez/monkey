#include "monkey/parser/statement.hh"

#include <memory>
#include <utility>

#include <cassert>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"
#include "monkey/parser/expression.hh"
#include "monkey/parser/parser.hh"

BEGIN_MONKEY_NAMESPACE

LetStatement::LetStatement(const IdentifierExpression _name,
                           std::unique_ptr<Expression> _value) noexcept
  : name{ std::move(_name) }
  , value{ std::move(_value) }
{
}

std::unique_ptr<LetStatement>
LetStatement::parse(Parser& parser) noexcept
{
    assert(parser.current_token().has_value());

    parser.next();
    IdentifierExpression name{ parser.current_token().value().literal };

    if (!parser.next_token_is(TokenType::Assign)) {
        return nullptr;
    }

    parser.next();
    std::unique_ptr<Expression> value = nullptr;

    while (parser.current_token().has_value() &&
           !parser.current_token_is(TokenType::Semicolon)) {
        // TODO: Evaluate the expression.
        parser.next();
    }

    return std::make_unique<LetStatement>(std::move(name), std::move(value));
}

// ----------------------------------------------------------------------------

ReturnStatement::ReturnStatement(std::unique_ptr<Expression> _value) noexcept
  : value{ std::move(_value) }
{
}

std::unique_ptr<ReturnStatement>
ReturnStatement::parse(Parser& parser) noexcept
{
    assert(parser.current_token().has_value());
    std::unique_ptr<Expression> value = nullptr;

    while (parser.current_token().has_value() &&
           !parser.current_token_is(TokenType::Semicolon)) {
        // TODO: Evaluate the expression.
        parser.next();
    }

    return std::make_unique<ReturnStatement>(std::move(value));
}

// ----------------------------------------------------------------------------

ExpressionStatement::ExpressionStatement(TokenType _type,
                                         std::unique_ptr<Expression> _value) noexcept
  : type{ _type }
  , value{ std::move(_value) }
{
}

std::unique_ptr<ExpressionStatement>
ExpressionStatement::parse(Parser& parser) noexcept
{
    assert(parser.current_token().has_value());
    TokenType type{ parser.current_token().value().type };
    std::unique_ptr<Expression> value = parser.parse_expression(Precedence::Lowest);
    assert(parser.current_token_is(TokenType::Semicolon));
    parser.next();
    return std::make_unique<ExpressionStatement>(type, std::move(value));
}

END_MONKEY_NAMESPACE
