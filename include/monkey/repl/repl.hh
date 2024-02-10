#pragma once

#include <iostream>

#include "monkey/common.hh"

BEGIN_MONKEY_NAMESPACE

void
repl(std::ostream& = std::cout, std::istream& = std::cin) noexcept;

END_MONKEY_NAMESPACE
