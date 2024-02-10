#include <string_view>

#include <cstddef>

#include <gtest/gtest.h>

#include "monkey/lexer/lexer.hh"
#include "monkey/parser/parser.hh"
#include "monkey/parser/statement.hh"

using namespace monkey;

TEST(ParserTestLetStatement, BasicAssertions)
{
    Lexer lexer{ R"XXX(
        let x = 5;
        let y = true;
        let foo = "bar";
    )XXX" };

    static const std::string_view expected[] = {
        "x",
        "y",
        "foo",
    };

    size_t size = sizeof expected / sizeof *expected;

    Parser parser{ lexer };
    Program program{};
    parser.parse_program(program);

    for (size_t i = 0; i < size; ++i) {
        const std::unique_ptr<Statement>& s = program.statements.at(i);
        const std::string_view& e = expected[i];
        ASSERT_EQ(s->get_type(), TokenType::Let);
        ASSERT_EQ(static_cast<const LetStatement*>(s.get())->name.value, e);
    }
}

TEST(ParserTestReturnStatement, BasicAssertions)
{
    Lexer lexer{ R"XXX(
        return 5;
        return true;
        return foo;
    )XXX" };
    Parser parser{ lexer };
    Program program{};
    parser.parse_program(program);

    for (const auto& s : program.statements) {
        ASSERT_EQ(s->get_type(), TokenType::Return);
        [[maybe_unused]] const auto& value =
          static_cast<const ReturnStatement*>(s.get())->value;

        // TODO: Recursively determine actual value?
    }
}

TEST(ParserTestInfixExpression, BasicAssertions)
{
}

TEST(ParserTestOperatorPrecedence, BasicAssertions)
{
    Lexer lexer{ R"XXX(
        true;
        false;
        3 > 5 == false;
        3 < 5 == true;
    )XXX" };
    [[maybe_unused]] const std::string_view expected[] = {
        "true;",
        "false;",
        "((3 > 5) == false);",
        "((3 < 5) == true);",
    };
}
