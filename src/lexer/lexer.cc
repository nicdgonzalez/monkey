#include "monkey/lexer/lexer.hh"

#include <algorithm>
#include <map>
#include <optional>
#include <string>
#include <string_view>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"

BEGIN_MONKEY_NAMESPACE

Lexer::Lexer(const std::string_view _input) noexcept
  : input_{ std::move(_input) }
  , position_{ input_.begin() }
{
}

void
Lexer::emplace(const std::string_view _new_input) noexcept
{
    input_ = std::move(_new_input);
    position_ = input_.begin();
}

bool
Lexer::peek(const char _c) noexcept
{
    bool matched = *position_ == _c;

    if (matched) {
        position_ += 1;
    }

    return matched;
}

namespace detail {

constexpr bool
identifier(const char _c) noexcept
{
    return isalnum(_c) || _c == '_';
}

std::optional<char>
get_escape_sequence(char escaped) noexcept
{
    // clang-format off
    static const std::map<char, char> escape_sequence_map{
        {'a', '\a'},
        {'b', '\b'},
        {'f', '\f'},
        {'n', '\n'},
        {'r', '\r'},
        {'t', '\t'},
        {'v', '\v'},
        {'\\', '\\'},
        {'\'', '\''},
        {'\"', '\"'},
    };
    // clang-format on

    auto escape_sequence = escape_sequence_map.find(escaped);

    if (escape_sequence == escape_sequence_map.end()) {
        return std::nullopt;
    }

    return escape_sequence->second;
}

} // namespace detail

std::optional<Token>
Lexer::next() noexcept
{
    // Skip any whitespaces before the next token.
    position_ = std::find_if_not(position_, input_.end(), ::isspace);

    if (position_ >= input_.end()) {
        return std::nullopt; // We have reached the end of the file.
    }

    const std::string_view::const_iterator start_position = position_++;

    switch (*start_position) {
        /* Operators */
        case '=':
            return peek('=') ? Equal : Assign;
        case '!':
            return peek('=') ? NotEqual : Bang;
        case '+':
            return Plus;
        case '-':
            return Minus;
        case '*':
            return Asterisk;
        case '/':
            return Slash;
        case '<':
            return peek('=') ? LessEqual : LessThan;
        case '>':
            return peek('=') ? GreaterEqual : GreaterThan;
        /* Delimiters */
        case ',':
            return Comma;
        case ';':
            return Semicolon;
        case ':':
            return Colon;
        case '(':
            return LParenthesis;
        case ')':
            return RParenthesis;
        case '{':
            return LBrace;
        case '}':
            return RBrace;
        case '[':
            return LBracket;
        case ']':
            return RBracket;
        case '"':
            return get_string_token();
        default:
            break;
    }

    // Keyword or Identifier
    if (isalpha(*start_position) || *start_position == '_') {
        position_ = std::find_if_not(position_, input_.end(), detail::identifier);

        const std::string_view literal{ start_position, position_ };
        const auto keyword = keyword_map.find(literal);

        if (keyword != keyword_map.end()) {
            return Token{ keyword->second, std::move(literal) };
        }

        return Token{ TokenType::Identifier, std::move(literal) };
    }

    // Integer
    if (::isdigit(*start_position)) {
        position_ = std::find_if_not(position_, input_.end(), ::isdigit);
        std::string_view literal{ start_position, position_ };
        return Token{ TokenType::Integer, std::move(literal) };
    }

    // Unrecognized token
    return Token{ TokenType::Illegal, { start_position, position_ } };
}

// TODO: Throw exceptions for bad input?
std::optional<Token>
Lexer::get_string_token()
{
    std::string str{};
    char c = '\0';

    while (position_ < input_.end() && (c = *position_++) != '"') {
        if (c == '\\') {
            if (position_ == input_.end()) {
                return std::nullopt; // Unterminated string.
            }

            std::optional<char> escape = detail::get_escape_sequence(*position_++);

            if (!escape.has_value()) {
                return std::nullopt; // Invalid escape sequence.
            }

            str += escape.value();
        } else {
            str += c;
        }
    }

    if (c != '"') {
        return std::nullopt; // Unterminated string.
    }

    return Token{ TokenType::String, std::move(str) };
}

END_MONKEY_NAMESPACE
