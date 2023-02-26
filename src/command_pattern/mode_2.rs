type FnPtr = fn() -> &'static str;

///
/// More light weight.
/// We only define a variable to save a command
pub struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

#[derive(Default)]
pub struct Schema {
    commands: Vec<Command>,
}

impl Schema {
    pub fn add_migration(&mut self, command: Command) {
        self.commands.push(command)
    }

    pub fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    pub fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn use_pattern_command_mode() {
        fn add_field() -> &'static str {
            "add field"
        }

        fn remove_field() -> &'static str {
            "remove field"
        }

        let mut schema = Schema::default();
        let add_files = Command {
            execute: add_field,
            rollback: remove_field,
        };
        schema.add_migration(Command {
            execute: || "create table",
            rollback: || "drop table",
        });
        schema.add_migration(add_files);
        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}
