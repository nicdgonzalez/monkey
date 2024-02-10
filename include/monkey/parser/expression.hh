#pragma once

#include <memory>
#include <string_view>

#include <cassert>
#include <vector>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"
#include "monkey/parser/ast.hh"

BEGIN_MONKEY_NAMESPACE

extern std::unique_ptr<Expression>
parse_grouped_expression(Parser&) noexcept;

struct IdentifierExpression final : public Expression
{
    static constexpr TokenType type{ TokenType::Identifier };
    const std::string_view value;

    static std::unique_ptr<IdentifierExpression> parse(Parser&) noexcept;

    explicit IdentifierExpression(const std::string_view) noexcept;
    ~IdentifierExpression() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

struct IntegerLiteral final : public Expression
{
    static constexpr TokenType type{ TokenType::Integer };
    const int64_t value;

    static std::unique_ptr<IntegerLiteral> parse(Parser&) noexcept;

    explicit IntegerLiteral(int64_t) noexcept;
    ~IntegerLiteral() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

struct PrefixExpression final : public Expression
{
    TokenType type;
    const std::string_view op;
    std::unique_ptr<Expression> right;

    PrefixExpression(const TokenType,
                     const std::string_view,
                     std::unique_ptr<Expression>) noexcept;

    ~PrefixExpression() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }

    static std::unique_ptr<PrefixExpression> parse(Parser&) noexcept;
};

struct InfixExpression final : public Expression
{
    TokenType type;
    std::unique_ptr<Expression> left;
    const std::string_view op;
    std::unique_ptr<Expression> right;

    /**
     *
     */
    static std::unique_ptr<InfixExpression> parse(Parser&,
                                                  std::unique_ptr<Expression>) noexcept;

    InfixExpression(TokenType,
                    std::unique_ptr<Expression>,
                    const std::string_view,
                    std::unique_ptr<Expression>) noexcept;

    /**
     *
     */
    ~InfixExpression() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

// ----------------------------------------------------------------------------

template<TokenType T>
concept BooleanToken = (T == TokenType::True || T == TokenType::False);

template<TokenType T>
    requires BooleanToken<T>
struct BooleanExpression final : public Expression
{
    static constexpr TokenType type{ T };
    const bool value;

    static std::unique_ptr<BooleanExpression> parse(Parser&) noexcept;

    explicit BooleanExpression(bool) noexcept;
    ~BooleanExpression() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

template<TokenType T>
    requires BooleanToken<T>
BooleanExpression<T>::BooleanExpression(bool _value) noexcept
  : value{ _value }
{
}

template<TokenType T>
    requires BooleanToken<T>
std::unique_ptr<BooleanExpression<T>>
BooleanExpression<T>::parse(Parser& parser) noexcept
{
    assert(parser.current_token().has_value());
    const Token& c = parser.current_token().value();
    return std::make_unique<BooleanExpression>(c.type == TokenType::True);
}

// ----------------------------------------------------------------------------

struct BlockStatement final : public Statement
{
    static constexpr TokenType type{ TokenType::LBrace };
    std::vector<std::unique_ptr<Statement>> statements{};

    static std::unique_ptr<BlockStatement> parse(Parser&) noexcept;

    explicit BlockStatement() noexcept = default;
    explicit BlockStatement(const BlockStatement&) noexcept = default;
    explicit BlockStatement(BlockStatement&&) noexcept = default;
    ~BlockStatement() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

// ----------------------------------------------------------------------------

struct IfExpression final : public Expression
{
    static constexpr TokenType type{ TokenType::If };
    std::unique_ptr<Expression> condition;
    std::unique_ptr<BlockStatement> consequence;
    std::unique_ptr<BlockStatement> alternative;

    static std::unique_ptr<IfExpression> parse(Parser&) noexcept;

    IfExpression(std::unique_ptr<Expression>,
                 std::unique_ptr<BlockStatement>,
                 std::unique_ptr<BlockStatement> = nullptr) noexcept;
    ~IfExpression() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

// ----------------------------------------------------------------------------

struct FunctionLiteral final : public Expression
{
    static constexpr TokenType type{ TokenType::Function };
    std::vector<std::unique_ptr<IdentifierExpression>> parameters{};
    std::unique_ptr<BlockStatement> body;

    static std::unique_ptr<FunctionLiteral> parse(Parser&) noexcept;

    explicit FunctionLiteral(std::unique_ptr<BlockStatement>) noexcept;
    ~FunctionLiteral() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

extern void
parse_function_parameters(Parser&, FunctionLiteral&) noexcept;

END_MONKEY_NAMESPACE
