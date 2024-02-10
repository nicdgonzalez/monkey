#include "monkey/parser/expression.hh"

#include <cstdlib>
#include <memory>
#include <string_view>
#include <utility>

#include <cassert>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"
#include "monkey/parser/parser.hh"

BEGIN_MONKEY_NAMESPACE

std::unique_ptr<Expression>
parse_grouped_expression(Parser& parser) noexcept
{
    parser.next();
    std::unique_ptr<Expression> e = parser.parse_expression(Precedence::Lowest);

    if (!parser.next_token_is(TokenType::RParenthesis)) {
        return nullptr;
    }

    return e;
}

// ----------------------------------------------------------------------------

IdentifierExpression::IdentifierExpression(const std::string_view _value) noexcept
  : value{ std::move(_value) }
{
}

std::unique_ptr<IdentifierExpression>
IdentifierExpression::parse(Parser& parser) noexcept
{
    assert(parser.current_token_is(TokenType::Identifier));
    std::string_view value{ parser.current_token().value().literal };
    return std::make_unique<IdentifierExpression>(std::move(value));
}

// ----------------------------------------------------------------------------

IntegerLiteral::IntegerLiteral(int64_t _value) noexcept
  : value{ _value }
{
}

std::unique_ptr<IntegerLiteral>
IntegerLiteral::parse(Parser& parser) noexcept
{
    assert(parser.current_token_is(TokenType::Integer));
    int64_t value = atoll(parser.current_token().value().literal.data());
    return std::make_unique<IntegerLiteral>(value);
}

// ----------------------------------------------------------------------------

PrefixExpression::PrefixExpression(const TokenType _type,
                                   const std::string_view _op,
                                   std::unique_ptr<Expression> _right) noexcept
  : type{ _type }
  , op{ std::move(_op) }
  , right{ std::move(_right) }
{
}

std::unique_ptr<PrefixExpression>
PrefixExpression::parse(Parser& parser) noexcept
{
    assert(parser.current_token().has_value());
    const Token& c = parser.current_token().value();
    auto expression = std::make_unique<PrefixExpression>(c.type, c.literal, nullptr);
    parser.next();
    expression->right = parser.parse_expression(Precedence::Prefix);
    return expression;
}

// ----------------------------------------------------------------------------

InfixExpression::InfixExpression(const TokenType _type,
                                 std::unique_ptr<Expression> _left,
                                 const std::string_view _op,
                                 std::unique_ptr<Expression> _right) noexcept
  : type{ _type }
  , left{ std::move(_left) }
  , op{ std::move(_op) }
  , right{ std::move(_right) }
{
}

std::unique_ptr<InfixExpression>
InfixExpression::parse(Parser& parser, std::unique_ptr<Expression> _left) noexcept
{
    assert(parser.current_token().has_value());
    const Token& current = parser.current_token().value();
    Precedence precedence = Parser::get_precedence(current.type);
    parser.next();
    std::unique_ptr<Expression> right = parser.parse_expression(precedence);
    return std::make_unique<InfixExpression>(current.type, std::move(_left), current.literal,
                                             std::move(right));
}

// ----------------------------------------------------------------------------

std::unique_ptr<BlockStatement>
BlockStatement::parse(Parser& parser) noexcept
{
    assert(parser.current_token().has_value());
    auto block = std::make_unique<BlockStatement>();

    while (parser.current_token().has_value() && !parser.current_token_is(TokenType::RBrace)) {
        std::unique_ptr<Statement> statement = parser.parse_statement();

        if (statement.get() != nullptr) {
            block->statements.push_back(std::move(statement));
        }

        parser.next();
    }

    return block;
}

// ----------------------------------------------------------------------------

IfExpression::IfExpression(std::unique_ptr<Expression> _condition,
                           std::unique_ptr<BlockStatement> _consequence,
                           std::unique_ptr<BlockStatement> _alternative) noexcept
  : condition{ std::move(_condition) }
  , consequence{ std::move(_consequence) }
  , alternative{ std::move(_alternative) }
{
}

std::unique_ptr<IfExpression>
IfExpression::parse(Parser& parser) noexcept
{
    assert(parser.current_token_is(TokenType::If));

    if (!parser.next_token_is(TokenType::LParenthesis)) {
        return nullptr;
    }

    parser.next();
    std::unique_ptr<Expression> condition = parser.parse_expression(Precedence::Lowest);

    if (!parser.next_token_is(TokenType::RParenthesis)) {
        return nullptr;
    }

    parser.next();

    if (!parser.next_token_is(TokenType::LBrace)) {
        return nullptr;
    }

    parser.next();
    std::unique_ptr<BlockStatement> consequence = BlockStatement::parse(parser);
    std::unique_ptr<BlockStatement> alternative = nullptr;

    if (parser.next_token_is(TokenType::Else)) {
        parser.next();

        if (!parser.next_token_is(TokenType::LBrace)) {
            return nullptr;
        }

        parser.next();
        alternative = BlockStatement::parse(parser);
    }

    return std::make_unique<IfExpression>(std::move(condition), std::move(consequence),
                                          std::move(alternative));
}

// ----------------------------------------------------------------------------

FunctionLiteral::FunctionLiteral(std::unique_ptr<BlockStatement> _body) noexcept
  : body{ std::move(_body) }
{
}

std::unique_ptr<FunctionLiteral>
FunctionLiteral::parse(Parser& parser) noexcept
{
    assert(parser.current_token_is(TokenType::Function));

    if (!parser.next_token_is(TokenType::LParenthesis)) {
        return nullptr;
    }

    parser.next();
    auto fn = std::make_unique<FunctionLiteral>(/*_body=*/nullptr);
    parse_function_parameters(parser, *fn);

    if (!parser.next_token_is(TokenType::LBrace)) {
        return nullptr;
    }

    parser.next();
    fn->body = BlockStatement::parse(parser);

    return fn;
}

void
parse_function_parameters(Parser& parser, FunctionLiteral& fn) noexcept
{
    assert(parser.current_token_is(TokenType::LParenthesis));

    // Handle case where there are no function parameters.
    if (parser.next_token_is(TokenType::RParenthesis)) {
        parser.next();
        return;
    }

    parser.next();
    const std::optional<Token>& current = parser.current_token();
    std::vector<std::string_view> parameter_names{};

    while (parser.next_token_is(TokenType::Comma)) {
        parameter_names.emplace_back(current.value().literal);
        parser.next(); // Identifier => Comma
        parser.next(); // Comma => Next Identifier
    }

    if (!parser.next_token_is(TokenType::RParenthesis)) {
        return;
    }

    for (auto& name : parameter_names) {
        fn.parameters.emplace_back(std::make_unique<IdentifierExpression>(std::move(name)));
    }

    // TODO: There is currently no way to tell whether the function returned
    // because there were no function parameters, or if it left because there
    // was an error with the parameter names...
}

END_MONKEY_NAMESPACE
