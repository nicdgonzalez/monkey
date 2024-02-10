#include "monkey/parser/parser.hh"

#include <functional>
#include <map>
#include <memory>
#include <optional>

#include <cassert>

#include "monkey/common.hh"
#include "monkey/lexer/lexer.hh"
#include "monkey/lexer/token.hh"
#include "monkey/parser/ast.hh"
#include "monkey/parser/expression.hh"
#include "monkey/parser/statement.hh"

BEGIN_MONKEY_NAMESPACE

const std::map<TokenType, Precedence> precedence_map{
    { TokenType::Equal, Precedence::Equals },
    { TokenType::NotEqual, Precedence::Equals },
    { TokenType::LessThan, Precedence::LessGreater },
    { TokenType::GreaterThan, Precedence::LessGreater },
    { TokenType::Plus, Precedence::Sum },
    { TokenType::Minus, Precedence::Sum },
    { TokenType::Asterisk, Precedence::Product },
    { TokenType::Slash, Precedence::Product },
};

// clang-format off
using PrefixParseFn = std::function<std::unique_ptr<Expression>(Parser&)>;
using InfixParseFn = std::function<std::unique_ptr<Expression>(Parser&, std::unique_ptr<Expression>)>;
// clang-format on

const std::map<TokenType, PrefixParseFn> prefix_parse_fn_map{
    { TokenType::Identifier, IdentifierExpression::parse },
    { TokenType::Bang, PrefixExpression::parse },
    { TokenType::Minus, PrefixExpression::parse },
    { TokenType::LParenthesis, parse_grouped_expression },
    { TokenType::If, IfExpression::parse },
};

const std::map<TokenType, InfixParseFn> infix_parse_fn_map{
    { TokenType::Plus, InfixExpression::parse },
    { TokenType::Minus, InfixExpression::parse },
    { TokenType::Asterisk, InfixExpression::parse },
    { TokenType::Slash, InfixExpression::parse },
    { TokenType::Equal, InfixExpression::parse },
    { TokenType::NotEqual, InfixExpression::parse },
    { TokenType::LessThan, InfixExpression::parse },
    { TokenType::GreaterThan, InfixExpression::parse },
};

Parser::Parser(Lexer& _lexer) noexcept
  : lexer_{ _lexer }
  , current_token_{ std::nullopt }
  , next_token_{ lexer_.next() }
{
    next();
}

Precedence
Parser::get_precedence(const TokenType& _type) noexcept
{
    auto precedence = precedence_map.find(_type);

    if (precedence == precedence_map.end()) {
        return Precedence::Lowest;
    }

    return precedence->second;
}

void
Parser::next() noexcept
{
    auto swap = [&](std::optional<Token>& token, std::optional<Token>& next) {
        if (next.has_value()) {
            token.emplace(std::move(next.value()));
        } else {
            token.reset();
        }
    };

    std::optional<Token> tmp = lexer_.next();
    swap(current_token_, next_token_);
    swap(next_token_, tmp);
}

void
Parser::parse_program(Program& _program) noexcept
{
    std::unique_ptr<Statement> statement = nullptr;

    while (current_token_.has_value()) {
        statement = parse_statement();

        if (statement.get() != nullptr) {
            _program.statements.push_back(std::move(statement));
        }

        next();
    }
}

std::unique_ptr<Statement>
Parser::parse_statement() noexcept
{
    assert(current_token_.has_value());

    switch (current_token_.value().type) {
        case TokenType::Let:
            return Statement::create<LetStatement>(*this);
        case TokenType::Return:
            return Statement::create<ReturnStatement>(*this);
        default:
            return Statement::create<ExpressionStatement>(*this);
    }
}

std::unique_ptr<Expression>
Parser::parse_expression(const Precedence _precedence) noexcept
{
    auto prefix = prefix_parse_fn_map.find(current_token_.value().type);

    if (prefix == prefix_parse_fn_map.end()) {
        return nullptr; // No function registered for current token type.
    }

    std::unique_ptr<Expression> left_expression = prefix->second(*this);

    while (!next_token_is(TokenType::Semicolon) &&
           _precedence < get_precedence(next_token_.value().type)) {
        auto infix = infix_parse_fn_map.find(next_token_.value().type);

        if (infix == infix_parse_fn_map.end()) {
            return left_expression;
        }

        next();
        left_expression = infix->second(*this, std::move(left_expression));
    }

    return left_expression;
}

END_MONKEY_NAMESPACE
