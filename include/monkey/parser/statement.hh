#pragma once

#include <memory>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"
#include "monkey/parser/ast.hh"
#include "monkey/parser/expression.hh"

BEGIN_MONKEY_NAMESPACE

struct LetStatement final : public Statement
{
    static constexpr TokenType type{ TokenType::Let };
    IdentifierExpression name;
    std::unique_ptr<Expression> value;

    static std::unique_ptr<LetStatement> parse(Parser&) noexcept;

    LetStatement(const IdentifierExpression, std::unique_ptr<Expression>) noexcept;
    ~LetStatement() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

struct ReturnStatement final : public Statement
{
    static constexpr TokenType type{ TokenType::Return };
    std::unique_ptr<Expression> value;

    static std::unique_ptr<ReturnStatement> parse(Parser&) noexcept;

    explicit ReturnStatement(std::unique_ptr<Expression>) noexcept;
    ~ReturnStatement() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

struct ExpressionStatement final : public Statement
{
    TokenType type;
    std::unique_ptr<Expression> value;

    static std::unique_ptr<ExpressionStatement> parse(Parser&) noexcept;

    ExpressionStatement(TokenType, std::unique_ptr<Expression>) noexcept;
    ~ExpressionStatement() noexcept = default;

    constexpr const TokenType& get_type() const noexcept override
    {
        return type;
    }
};

END_MONKEY_NAMESPACE
