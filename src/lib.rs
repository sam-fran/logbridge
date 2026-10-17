/// Outline for a Logger instance
pub enum Outline {
	Single,
	Double,
}

pub enum Level {
	DEBUG,
	INFO,
	NOTICE,
	WARN,
	ERROR,
	EMERG,
}

pub struct Logger {
	outline: Outline,
}