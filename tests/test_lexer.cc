#include <optional>
#include <string_view>

#include <gtest/gtest.h>

#include "monkey/lexer/lexer.hh"
#include "monkey/lexer/token.hh"

using namespace monkey;

TEST(LexerTestSimpleInput, BasicAssertions)
{
    Lexer lexer{ R"XXX(
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
    )XXX" };

    // Expected Tokens {{{
    // clang-format off
    const std::optional<const Token> expected[] = {
        Let,
        Token{ TokenType::Identifier, "five" },
        Assign,
        Token{ TokenType::Integer, "5" },
        Semicolon,
        Let,
        Token{ TokenType::Identifier, "ten" },
        Assign,
        Token{ TokenType::Integer, "10" },
        Semicolon,
        Let,
        Token{ TokenType::Identifier, "add" },
        Assign,
        Function,
        LParenthesis,
        Token{ TokenType::Identifier, "x" },
        Comma,
        Token{ TokenType::Identifier, "y" },
        RParenthesis,
        LBrace,
        Token{ TokenType::Identifier, "x" },
        Plus,
        Token{ TokenType::Identifier, "y" },
        Semicolon,
        RBrace,
        Semicolon,
        Let,
        Token{ TokenType::Identifier, "result" },
        Assign,
        Token{ TokenType::Identifier, "add" },
        LParenthesis,
        Token{ TokenType::Identifier, "five" },
        Comma,
        Token{ TokenType::Identifier, "ten" },
        RParenthesis,
        Semicolon,
        Bang,
        Minus,
        Slash,
        Asterisk,
        Token{ TokenType::Integer, "5" },
        Semicolon,
        Token{ TokenType::Integer, "5" },
        LessThan,
        Token{ TokenType::Integer, "10" },
        GreaterThan,
        Token{ TokenType::Integer, "5" },
        Semicolon,
        If,
        LParenthesis,
        Token{ TokenType::Integer, "5" },
        LessThan,
        Token{ TokenType::Integer, "10" },
        RParenthesis,
        LBrace,
        Return,
        True,
        Semicolon,
        RBrace,
        Else,
        LBrace,
        Return,
        False,
        Semicolon,
        RBrace,
        Token{ TokenType::Integer, "10" },
        Equal,
        Token{ TokenType::Integer, "10" },
        Semicolon,
        Token{ TokenType::Integer, "10" },
        NotEqual,
        Token{ TokenType::Integer, "9" },
        Semicolon,
        std::nullopt,
    };
    // clang-format on
    // }}}

    std::optional<Token> t = std::nullopt;

    for (const auto& e : expected) {
        try {
            t.emplace(lexer.next().value());
        } catch (const std::bad_optional_access& exc) {
            t.reset();
        }

        if (e.has_value()) {
            ASSERT_TRUE(t.has_value());
            ASSERT_EQ(e.value().literal, t.value().literal);
            ASSERT_EQ(e.value().type, t.value().type);
        } else {
            ASSERT_EQ(!e.has_value(), !t.has_value());
        }
    }
}
