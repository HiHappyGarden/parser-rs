use crate::context::AtContext;
use crate::{AtError, AtResult, Args};

/*
AT+CMD (esecuzione)

AT+CMD? (query)

AT+CMD=? (test)

AT+CMD= (set)
 */

enum AtForm<'a> {
    Exec,
    Query,
    Test,
    Set(Args<'a>),
}

pub struct AtParser<'a, T>
where
    T: AtContext {
    pub commands: &'a mut [(&'static str, &'a mut T)],
}

impl<'a, T> AtParser<'a, T>
where
    T: AtContext {

    pub fn new() -> Self {
        Self { commands: & mut [] }
    }

    pub fn set_commands(&mut self, commands: &'a mut [(&'static str, &'a mut T)]) {
        self.commands = commands;
    }

    pub fn execute(&mut self, input: &str) -> AtResult<'static> {
        let input = input.trim();
        let (name, form) = parse(input)?;

        let (_, module) = self.commands
            .iter_mut()
            .find(|(n, _)| *n == name)
            .ok_or(AtError::UnknownCommand)?;

        match form {
            AtForm::Exec => module.exec(),
            AtForm::Query => module.query(),
            AtForm::Test => module.test(),
            AtForm::Set(args) => module.set(args),
        }
    }
}

fn parse<'a>(input: &'a str) -> Result<(&'a str, AtForm<'a>), AtError> {
    let input = input.trim();

    if let Some(cmd) = input.strip_suffix("=?") {
        Ok((cmd, AtForm::Test))
    } else if let Some(cmd) = input.strip_suffix('?') {
        Ok((cmd, AtForm::Query))
    } else if let Some((cmd, args)) = input.split_once('=') {
        Ok((cmd, AtForm::Set(Args { raw: args })))
    } else {
        Ok((input, AtForm::Exec))
    }
}