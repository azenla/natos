use crate::shell::command::CommandList;
use anyhow::{Result, anyhow};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::rc::Rc;

pub struct Shell {
    commands: Rc<CommandList>,
    editor: DefaultEditor,
}

impl Shell {
    pub fn new(commands: CommandList) -> Result<Self> {
        Ok(Self {
            commands: Rc::new(commands),
            editor: DefaultEditor::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let raw_line = match self.editor.readline("> ") {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => {
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    std::process::exit(0);
                }
                Err(error) => {
                    return Err(anyhow!(error));
                }
            };
            let line = shlex::split(&raw_line).unwrap_or_else(|| vec![raw_line.to_string()]);
            if line.is_empty() {
                continue;
            }
            self.editor.add_history_entry(raw_line)?;
            self.process(line)?;
        }
    }

    pub fn process(&mut self, line: Vec<String>) -> Result<()> {
        if line.is_empty() {
            return Ok(());
        }
        let name = line[0].as_str();

        let args = line
            .iter()
            .skip(1)
            .map(|it| it.as_str())
            .collect::<Vec<_>>();

        let commands = self.commands.clone();

        let mut did_handle = false;
        for command in commands.iter() {
            if !command.can_handle(name) {
                continue;
            }

            did_handle = true;
            match command.run(self, &args) {
                Ok(_) => {
                    continue;
                }

                Err(error) => {
                    println!("error: {error}");
                }
            }
            break;
        }

        if !did_handle {
            println!("unknown command: {name}");
        }
        Ok(())
    }

    pub fn commands(&self) -> Rc<CommandList> {
        self.commands.clone()
    }

    pub fn editor(&mut self) -> &mut DefaultEditor {
        &mut self.editor
    }
}
