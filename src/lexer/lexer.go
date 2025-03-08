package lexer

import "monkey/token"

type Lexer struct {
	input        string
	position     int  // current position in input (points to current char)
	readPosition int  // current reading position in input (after current char)
	ch           byte // current char under examination
}

func New(input string) *Lexer {
	l := &Lexer{input: input}
	// Set the lexer to a working state before handing it off to the user.
	l.readChar()
	return l
}

// Return the next character and advance our position in the input string.
func (l *Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		// If we go off the end of the input, set `ch` to NULL.
		l.ch = 0
	} else {
		// Otherwise, read the next character.
		l.ch = l.input[l.readPosition]
	}

	// Advance both iterators by 1.
	l.position = l.readPosition
	l.readPosition += 1
}

func (l *Lexer) NextToken() token.Token {
	var tok token.Token

	switch l.ch {
	case '=':
		tok = newToken(token.ASSIGN, l.ch)
	case ';':
		tok = newToken(token.SEMICOLON, l.ch)
	case '(':
		tok = newToken(token.LPARENTHESIS, l.ch)
	case ')':
		tok = newToken(token.RPARENTHESIS, l.ch)
	case ',':
		tok = newToken(token.COMMA, l.ch)
	case '+':
		tok = newToken(token.PLUS, l.ch)
	case '{':
		tok = newToken(token.LBRACE, l.ch)
	case '}':
		tok = newToken(token.RBRACE, l.ch)
	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
	}

	l.readChar()
	return tok
}

func newToken(tokenType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}
