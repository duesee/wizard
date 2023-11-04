use std::{fmt::Display, str::FromStr};

use dialoguer::{theme::ColorfulTheme, Input};
pub use wizard_derive::WizardDerive;

pub trait Wizard: Sized {
    fn prompt(msg: &str) -> Self;
}

impl<T> Wizard for T
where
    T: FromStr + Display + Clone,
    T::Err: Display,
{
    fn prompt(msg: &str) -> Self {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .allow_empty(true)
            .interact_text()
            .unwrap()
    }
}
