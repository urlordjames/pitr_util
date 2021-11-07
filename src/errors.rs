#[derive(Debug)]
pub enum FromStringError {
	UnsupportedSaveVersion(u8),
	UnsupportedGameVersion(String),
	ParseError(serde_json::Error)
}

#[derive(Debug)]
pub enum FromFileError {
	FileError(std::io::Error),
	FromStringError(FromStringError)
}
