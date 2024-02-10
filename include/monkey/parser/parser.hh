#pragma once

#include <memory>
#include <optional>
#include <string_view>
#include <vector>

#include <cstdint>

#include "monkey/common.hh"
#include "monkey/lexer/lexer.hh"
#include "monkey/lexer/token.hh"

BEGIN_MONKEY_NAMESPACE

enum class Precedence : uint8_t
{
    Lowest = 1,
    Equals,      // == or !=
    LessGreater, // < or >
    Sum,         // + or -
    Product,     // * or /
    Prefix,      // -x or !x
    Call,        // foo()
};

struct Expression;
struct Statement;

struct Program final
{
    std::vector<std::unique_ptr<Statement>> statements{};
};

class Parser final
{
  private:
    Lexer& lexer_;
    std::optional<Token> current_token_;
    std::optional<Token> next_token_;
    std::vector<std::string_view> errors_{};

  public:
    static Precedence get_precedence(const TokenType&) noexcept;

    explicit Parser(Lexer&) noexcept;
    void next() noexcept;
    void parse_program(Program&) noexcept;
    std::unique_ptr<Statement> parse_statement() noexcept;
    std::unique_ptr<Expression> parse_expression(const Precedence) noexcept;

    constexpr const Lexer& lexer() const noexcept
    {
        return lexer_;
    }

    constexpr const std::optional<Token>& current_token() const noexcept
    {
        return current_token_;
    }

    constexpr bool current_token_is(const TokenType _t) const noexcept
    {
        return current_token_.has_value() && current_token_.value().type == _t;
    }

    constexpr const std::optional<Token>& next_token() const noexcept
    {
        return current_token_;
    }

    constexpr bool next_token_is(const TokenType _t) const noexcept
    {
        return next_token_.has_value() && next_token_.value().type == _t;
    }

    constexpr const std::vector<std::string_view>& errors() const noexcept
    {
        return errors_;
    }
};

END_MONKEY_NAMESPACE
