#include "monkey/repl/repl.hh"

int
main(const int argc, const char* const* argv)
{
    if (argc < 2) {
        monkey::repl();
        return 0;
    }
}
