use clap::Command;
use clap_complete::{generate, Generator, Shell};
use std::io::{self};

mod config;
mod project;

const ZSH_COMPLETION: &str = r#"#compdef skp

 _skp_project_completion() {
     local values
     values=(${(f)"$(skp ls 2>/dev/null)"})
     _describe 'values' values
 }

 _skp() {
     local line

    eval "$(skp completions zsh)"
    functions[_skp_real]="${functions[_skporgcompletions]}"

     _arguments -C \
         "1: :->cmds" \
         "*::arg:->args"

     case "$state" in
         args)
             case $line[1] in
                 project)
                     _skp_project_completion
                     return
                     ;;
                 *)
                     _skp_real "$@"
                     return
                     ;;
             esac
             ;;
         *)
              _skp_real "$@"
              return
             ;;
     esac
 }

 if [[ -n ${ZSH_VERSION-} ]]; then
     compdef _skp skp

     # FIXME This should be given by the user
     p() { eval "$(skp project $@)"; }
     compdef _skp_project_completion p
 fi"#;

fn build_cli(name: &'static str) -> Command {
    Command::new(name)
        .subcommand(
            Command::new("complete")
                .about("Generate shell completions")
                .arg(
                    clap::Arg::new("shell")
                        .value_parser(clap::value_parser!(Shell))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    clap::Arg::new("shell")
                        .value_parser(clap::value_parser!(Shell))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("project")
                .about("Move to a project")
                .arg(clap::Arg::new("name").required(true)),
        )
        .subcommand(Command::new("ls").about("List projects"))
}

fn print_completions<G: Generator>(gen1: G, cmd: &mut Command) {
    generate(gen1, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let ecfg = match config::env::load_env_cfg() {
        Ok(c) => c,
        Err(e) => {
            println!("ERROR {}", e);
            return;
        }
    };

    let cfg = match config::parser::parse(ecfg) {
        Ok(c) => c,
        Err(e) => {
            println!("ERROR {}", e);
            return;
        }
    };

    let mut cmd = build_cli("skp");
    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("complete", sub_matches)) => {
            let shell = sub_matches
                .get_one::<Shell>("shell")
                .expect("shell is required");
            if *shell == Shell::Zsh {
                println!("{}", ZSH_COMPLETION);
            } else {
                print_completions(*shell, &mut cmd);
            }
        }
        Some(("completions", sub_matches)) => {
            let shell = sub_matches
                .get_one::<Shell>("shell")
                .expect("shell is required");
            let mut cmd2 = build_cli("skporgcompletions");
            print_completions(*shell, &mut cmd2);
        }
        Some(("project", sub_matches)) => {
            if let Some(name) = sub_matches.get_one::<String>("name") {
                print!("{}", project::build_cmd(name.to_string(), cfg));
                // if let Some(projects) = cfg.project {
                //     match projects.iter().find(|proj| proj.name == *name) {
                //         Some(p) => print!("cd {}; tmux rename-window {}", p.path, p.name),
                //         None => return,
                //     }
                // }
            }
        }
        Some(("ls", _)) => {
            if let Some(projects) = cfg.project {
                for p in projects {
                    // let name = match p.name {
                    //     Some(s) => s,
                    //     None => "ajda".to_string(),
                    // };
                    println!("{}:{}", p.name, p.path)
                }
            }
        }
        _ => {
            cmd.print_help().unwrap();
        }
    }
}
