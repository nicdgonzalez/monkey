#pragma once

#include <memory>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"
#include "monkey/parser/parser.hh"

BEGIN_MONKEY_NAMESPACE

struct Node
{
    virtual ~Node() noexcept = 0;
    virtual const TokenType& get_type() const noexcept = 0;
};

template<typename T>
concept HasParseMethod = requires(T, Parser& p) {
    {
        T::parse(p)
    } -> std::convertible_to<std::unique_ptr<Node>>;
};

// ----------------------------------------------------------------------------

struct Expression : public Node
{
    virtual ~Expression() noexcept = 0;

    template<HasParseMethod T>
    static std::unique_ptr<Expression> create(Parser&) noexcept;
};

template<HasParseMethod T>
std::unique_ptr<Expression>
Expression::create(Parser& parser) noexcept
{
    return T::parse(parser);
}

// ----------------------------------------------------------------------------

struct Statement : public Node
{
    virtual ~Statement() noexcept = 0;

    template<HasParseMethod T>
    static std::unique_ptr<Statement> create(Parser& p) noexcept;
};

template<HasParseMethod T>
std::unique_ptr<Statement>
Statement::create(Parser& parser) noexcept
{
    return T::parse(parser);
}

END_MONKEY_NAMESPACE
