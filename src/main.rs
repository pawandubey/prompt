use colored::*;
use std::env;
use std::path::PathBuf;

fn main() {
    let username = whoami::username();
    let ruby_version = env::var("RUBY_VERSION").unwrap_or("".to_string());
    let current_dir = env::current_dir().unwrap_or(PathBuf::from(""));
    let current_dir_basename = match current_dir.file_stem() {
        Some(name) => name.to_str().unwrap_or(""),
        None => "",
    };
    let git_prompt = "master";

    let formatted_username = format!(" {} ", username)
        .bold()
        .white()
        .on_truecolor(0, 175, 135);

    let formatted_ruby_version = format!("[{}]", ruby_version).truecolor(0, 175, 135);

    let formatted_current_dir_basename = format!("[{}]", current_dir_basename)
        .bold()
        .truecolor(149, 120, 251);

    let formatted_git_prompt = format!("on {}", git_prompt.bold())
        .italic()
        .truecolor(91, 133, 242);

    let separator = ":".truecolor(0, 175, 135);

    println!(
        "{} {} {} {} {} {}",
        formatted_username,
        separator,
        formatted_ruby_version,
        separator,
        formatted_current_dir_basename,
        formatted_git_prompt
    );
    println!("{}", "★ 𝞴".truecolor(0, 175, 135));
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
