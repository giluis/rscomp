pub enum Error {
    LexerError(LexerError),
    ParserError(ParserError),
}



pub enum LexerError {
    UnrecognizedToken(char)
}

pub enum ParserError {
    NoMoreTokens, 
    CouldNotParseIntoStruct()
}
