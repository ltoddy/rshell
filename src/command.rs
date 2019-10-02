#[derive(Debug, PartialEq, Clone)]
pub enum BuiltinCommand {
    Quit,
    ShowCode,
    Clear,
}

impl From<String> for BuiltinCommand {
    fn from(input: String) -> Self {
        match input.as_str() {
            ":quit" | ":exit" => BuiltinCommand::Quit,
            ":code" => BuiltinCommand::ShowCode,
            ":clear" => BuiltinCommand::Clear,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BuiltinCommand;

    #[test]
    fn test_builtin_command_enum_basic() {
        let input = String::from(":quit");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::Quit);

        let input = String::from(":exit");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::Quit);

        let input = String::from(":code");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::ShowCode);

        let input = String::from(":clear");
        assert_eq!(BuiltinCommand::from(input), BuiltinCommand::Clear);
    }
}
