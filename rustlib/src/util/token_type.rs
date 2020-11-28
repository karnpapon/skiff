#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
  PLUS,
  LEFTBRACE,
  LEFTSQBRACKET,
  RIGHTSQBRACKET,
  RIGHTBRACE,
  SEMICOLON,
  VERTICALLINE,

  NAME,
  BODY,
  BREF,
  HOST,
  TYPE,
  LINK,
  LIST,

  EOS, //end of scope.
  EOF,
}