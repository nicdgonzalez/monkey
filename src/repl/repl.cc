#include "monkey/repl/repl.hh"

#include <iostream>
#include <optional>
#include <ostream>
#include <string>
#include <string_view>

#if MONKEY_WINDOWS
#include <windows.h>
#else
#include <unistd.h>
#endif

#include "monkey/common.hh"
#include "monkey/lexer/lexer.hh"
#include "monkey/lexer/token.hh"

BEGIN_MONKEY_NAMESPACE

namespace detail {

std::string
get_current_user() noexcept
{
    std::string name;
    name.resize(256);

#if MONKEY_WINDOWS
    DWORD size = name.size();
    GetUserName(name.data(), &size);
#else
    getlogin_r(name.data(), name.max_size());
#endif

    return name;
}

} // namespace detail

void
repl(std::ostream& out, std::istream& in) noexcept
{
    static const std::string_view prompt{ ">>> " };
    Lexer lexer{};

    out << "Hello, " << detail::get_current_user() << "! "
        << "This is the Monkey programming language!\n"
        << "Feel free to type in commands.\n";

    auto get_next_token = [&](std::optional<Token>& current) {
        std::optional<Token> next = lexer.next();

        if (next.has_value()) {
            current.emplace(next.value());
        } else {
            current.reset();
        }
    };

    for (std::string line{}; out << prompt, std::getline(in, line);) {
        if (line.compare("exit") == 0) {
            break;
        }

        lexer.emplace(line);

        for (std::optional<Token> t = lexer.next(); t.has_value(); get_next_token(t)) {
            out << t.value() << '\n';
        }
    }
}

END_MONKEY_NAMESPACE
