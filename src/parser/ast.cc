#include "monkey/parser/ast.hh"

#include "monkey/common.hh"
#include "monkey/parser/parser.hh"

BEGIN_MONKEY_NAMESPACE

Node::~Node() noexcept
{
}

Expression::~Expression() noexcept
{
}

Statement::~Statement() noexcept
{
}

END_MONKEY_NAMESPACE
