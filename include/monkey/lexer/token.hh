#pragma once

#include <string>
#include <string_view>
#include <unordered_map>

#include <cstdint>

#include "monkey/common.hh"

BEGIN_MONKEY_NAMESPACE

enum class TokenType : std::uint8_t
{
    Illegal = 1,
    /* Items */
    Identifier,
    Integer,
    String,
    /* Operators */
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
    /* Delimiters */
    Comma,
    Semicolon,
    Colon,
    LParenthesis,
    RParenthesis,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    /* Keyword */
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
};

struct Token
{
    TokenType type;
    const std::string_view literal;

    constexpr explicit Token(const TokenType type,
                             const std::string_view literal = {}) noexcept
      : type{ type }
      , literal{ std::move(literal) }
    {
    }
};

// The following tokens have a consistent value for `literal`.
// Note: Illegal, Identifier, Integer, and String are not included.
constexpr Token Assign{ TokenType::Assign, "=" };
constexpr Token Plus{ TokenType::Plus, "+" };
constexpr Token Minus{ TokenType::Minus, "-" };
constexpr Token Bang{ TokenType::Bang, "!" };
constexpr Token Asterisk{ TokenType::Asterisk, "*" };
constexpr Token Slash{ TokenType::Slash, "/" };
constexpr Token LessThan{ TokenType::LessThan, "<" };
constexpr Token LessEqual{ TokenType::LessEqual, "<=" };
constexpr Token GreaterThan{ TokenType::GreaterThan, ">" };
constexpr Token GreaterEqual{ TokenType::GreaterEqual, ">=" };
constexpr Token Equal{ TokenType::Equal, "==" };
constexpr Token NotEqual{ TokenType::NotEqual, "!=" };
constexpr Token Comma{ TokenType::Comma, "," };
constexpr Token Semicolon{ TokenType::Semicolon, ";" };
constexpr Token Colon{ TokenType::Colon, ":" };
constexpr Token LParenthesis{ TokenType::LParenthesis, "(" };
constexpr Token RParenthesis{ TokenType::RParenthesis, ")" };
constexpr Token LBrace{ TokenType::LBrace, "{" };
constexpr Token RBrace{ TokenType::RBrace, "}" };
constexpr Token LBracket{ TokenType::LBracket, "[" };
constexpr Token RBracket{ TokenType::RBracket, "]" };
constexpr Token Function{ TokenType::Function, "fn" };
constexpr Token Let{ TokenType::Let, "let" };
constexpr Token True{ TokenType::True, "true" };
constexpr Token False{ TokenType::False, "false" };
constexpr Token If{ TokenType::If, "if" };
constexpr Token Else{ TokenType::Else, "else" };
constexpr Token Return{ TokenType::Return, "return" };

extern const std::unordered_map<std::string_view, TokenType> keyword_map;

END_MONKEY_NAMESPACE

namespace std {

string
to_string(const monkey::TokenType) noexcept;
ostream&
operator<<(ostream&, const monkey::Token) noexcept;

} // namespace std
