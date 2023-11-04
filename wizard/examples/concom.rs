//! Tool for "Conventional Commits"
//!
//! https://www.conventionalcommits.org/en/v1.0.0/conventionalcommits

use std::fmt::{Display, Formatter};

use wizard::{Wizard, WizardDerive};

#[derive(Debug, WizardDerive)]
struct Prompt {
    /// Type
    kind: Kind,
    /// Scope (optional)
    scope: String,
    /// Breaking change?
    breaking: YesNo,
    /// Short summary
    description: String,
    // TODO: This should be `Option<String>` and open an `EDITOR` session or so.
    /// Longer commit body (optional)
    body: String,
    // TODO:
    // /// Footers
    // footers: Vec<String>
}

#[derive(Debug, WizardDerive)]
enum Kind {
    /// fix      -- Bug fix
    Fix,
    /// feat     -- New feature
    Feat,
    /// build    -- Build system or dependencies
    Build,
    /// ci       -- CI
    Ci,
    /// chore    -- Routine work
    Chore,
    /// docs     -- Documentation
    Docs,
    /// perf     -- Performance
    Perf,
    /// refactor -- Neither bug fix nor feature
    Refactor,
    /// style    -- Stylistic
    Style,
    /// test     -- Work on tests
    Test,
    // TODO:
    // /// other
    // Other(String),
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fix => "fix",
                Self::Feat => "feat",
                Self::Build => "build",
                Self::Ci => "ci",
                Self::Chore => "chore",
                Self::Docs => "docs",
                Self::Perf => "perf",
                Self::Refactor => "refactor",
                Self::Style => "style",
                Self::Test => "test",
            }
        )
    }
}

#[derive(Debug, WizardDerive)]
enum YesNo {
    /// Yes
    Yes,
    /// No
    No,
}

fn main() {
    let prompt: Prompt = Wizard::prompt("Prompt");

    let scope = if prompt.scope.is_empty() {
        "".into()
    } else {
        format!("({})", prompt.scope)
    };

    let breaking = match prompt.breaking {
        YesNo::Yes => "!",
        YesNo::No => "",
    };

    let body = if prompt.body.is_empty() {
        "".into()
    } else {
        format!("\n\n{}", prompt.body)
    };

    println!(
        "{}{}{}: {}{}",
        prompt.kind, scope, breaking, prompt.description, body
    );
}
