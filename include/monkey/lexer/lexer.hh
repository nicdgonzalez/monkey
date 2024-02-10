#pragma once

#include <optional>
#include <string_view>

#include "monkey/common.hh"
#include "monkey/lexer/token.hh"

BEGIN_MONKEY_NAMESPACE

class Lexer final
{
  private:
    std::string_view input_;
    std::string_view::iterator position_;

  public:
    explicit Lexer(const std::string_view = {}) noexcept;
    void emplace(const std::string_view) noexcept;
    std::optional<Token> next() noexcept;

  private:
    bool peek(const char) noexcept;
    std::optional<Token> get_string_token();
};

END_MONKEY_NAMESPACE
