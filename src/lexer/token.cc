#include "monkey/lexer/token.hh"

#include <iomanip>
#include <string_view>
#include <unordered_map>

#include "monkey/common.hh"

BEGIN_MONKEY_NAMESPACE

// clang-format off
const std::unordered_map<std::string_view, TokenType> keyword_map = {
    { "fn", TokenType::Function },
    { "let", TokenType::Let },
    { "true", TokenType::True },
    { "false", TokenType::False },
    { "if", TokenType::If },
    { "else", TokenType::Else },
    { "return", TokenType::Return },
};
// clang-format on

END_MONKEY_NAMESPACE

namespace std {

string
to_string(const monkey::TokenType type) noexcept
{
    using enum monkey::TokenType;

    switch (type) {
        case Identifier:
            return "IDENTIFIER";
        case Integer:
            return "INTEGER";
        case String:
            return "STRING";
        case Assign:
            return "ASSIGN";
        case Plus:
            return "PLUS";
        case Minus:
            return "MINUS";
        case Bang:
            return "BANG";
        case Asterisk:
            return "ASTERISK";
        case Slash:
            return "SLASH";
        case LessThan:
            return "LESS_THAN";
        case LessEqual:
            return "LESS_EQUAL";
        case GreaterThan:
            return "GREATER_THAN";
        case GreaterEqual:
            return "GREATER_EQUAL";
        case Equal:
            return "EQUAL";
        case NotEqual:
            return "NOT_EQUAL";
        case Comma:
            return "COMMA";
        case Semicolon:
            return "SEMICOLON";
        case Colon:
            return "COLON";
        case LParenthesis:
            return "LPARENTHESIS";
        case RParenthesis:
            return "RPARENTHESIS";
        case LBrace:
            return "LBRACE";
        case RBrace:
            return "RBRACE";
        case LBracket:
            return "LBRACKET";
        case RBracket:
            return "RBRACKET";
        case Function:
            return "FUNCTION";
        case Let:
            return "LET";
        case True:
            return "TRUE";
        case False:
            return "FALSE";
        case If:
            return "IF";
        case Else:
            return "ELSE";
        case Return:
            return "RETURN";
        case Illegal:
            [[fallthrough]];
        default:
            return "ILLEGAL";
    }
}

ostream&
operator<<(ostream& out, const monkey::Token t) noexcept
{
    out << "Token(" << std::to_string(t.type) << ", " << std::quoted(t.literal) << ")";
    return out;
}

} // namespace std
