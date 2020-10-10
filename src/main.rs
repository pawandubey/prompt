use colored::*;
use std::env;
use std::path::PathBuf;

trait PromptColors: Colorize {
    fn prompt_green(self) -> ColoredString
    where
        Self: Sized,
    {
        self.truecolor(0, 175, 135)
    }

    fn on_prompt_green(self) -> ColoredString
    where
        Self: Sized,
    {
        self.on_truecolor(0, 175, 135)
    }

    fn prompt_purple(self) -> ColoredString
    where
        Self: Sized,
    {
        self.truecolor(149, 120, 251)
    }

    fn prompt_blue(self) -> ColoredString
    where
        Self: Sized,
    {
        self.truecolor(91, 133, 242)
    }
}

impl<'a> PromptColors for &'a str {}

impl PromptColors for ColoredString {}

fn main() {
    let username = whoami::username();
    let ruby_version = env::var("RUBY_VERSION").unwrap_or_else(|_| "".to_string());
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(""));
    let current_dir_basename = match current_dir.file_stem() {
        Some(name) => name.to_str().unwrap_or(""),
        None => "",
    };
    let git_prompt = "master";

    let formatted_username = format!(" {} ", username).bold().white().on_prompt_green();

    let formatted_ruby_version = format!("[{}]", ruby_version).prompt_green();

    let formatted_current_dir_basename =
        format!("[{}]", current_dir_basename).bold().prompt_purple();

    let formatted_git_prompt = format!("on {}", git_prompt.bold()).italic().prompt_blue();

    let separator = ":".prompt_green();

    println!(
        "{} {} {} {} {} {}",
        formatted_username,
        separator,
        formatted_ruby_version,
        separator,
        formatted_current_dir_basename,
        formatted_git_prompt
    );
    println!("{}", "★ 𝞴".prompt_green());
}

// Translation of the following bash script
// https://github.com/pawandubey/dotfiles/blob/7b7714dbf531052887b4a9df26c6a7c0447d5263/bash/.bashrc#L116-L142
//
// ## Custom bash prompt
// GIT_PS1_SHOWDIRTYSTATE=1
// GIT_PS1_SHOWUNTRACKEDFILES=1

// bold()                                 { ansi 1 "$@"; }
// italic()                               { ansi 3 "$@"; }
// underline()                            { ansi 4 "$@"; }
// strikethrough()                        { ansi 9 "$@"; }
// gitprompt()                            { ansi "38;5;069" "$@"; }
// green_foreground()                     { ansi "38;5;036" "$@"; }
// green_background_white_foreground()    { ansi "48;5;036;38" "$@"; }
// purple_bold_foreground()               { ansi "35;1" "$@"; }
// ansi()                                 { echo "\[\e[${1}m\]${*:2}\[\e[0m\]"; }

// normal=$(tput sgr0)

// function gitps1() {
//     echo "\$(__git_ps1 \"on $(bold %s)\")"
// }

// function ruby_version() {
//     if [ $RUBY_VERSION ] ; then
//         echo " : [$RUBY_VERSION] :"
//     fi
// }

// export PS1=`echo -e "\$(bold \$(green_background_white_foreground ' \u '))\$(green_foreground '\$(ruby_version)') \$(purple_bold_foreground ['\W']) \$(gitprompt \$(italic \$(gitps1)))\n\$(green_foreground ★ 𝞴) "`
