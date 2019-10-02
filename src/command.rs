#[derive(Debug, PartialEq, Clone)]
pub enum BuiltinCommand {
    Exit,
    ShowCode,
    Clear,

    Invalid(String),
}

impl From<String> for BuiltinCommand {
    fn from(input: String) -> Self {
        match input.as_str() {
            ":quit" | ":exit" => BuiltinCommand::Exit,
            ":code" => BuiltinCommand::ShowCode,
            ":clear" => BuiltinCommand::Clear,

            _ => BuiltinCommand::Invalid(input),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BuiltinCommand;

    #[test]
    fn test_builtin_command_enum_basic() {
        let input = String::from(":quit");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::Exit);

        let input = String::from(":exit");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::Exit);

        let input = String::from(":code");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::ShowCode);

        let input = String::from(":clear");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::Clear);
    }
}
