use handlebars::Handlebars;
use serde_json::json;

use super::config;
pub fn build_cmd(name: String, cfg: config::parser::Config) -> String {
    if let Some(projects) = cfg.project {
        let project: config::parser::Project = match projects.iter().find(|proj| proj.name == name)
        {
            Some(p) => p.clone(),
            None => return "".to_string(),
        };
        let reg = Handlebars::new();

        let name = project.name.clone();
        let path = project.path.clone();

        let mut cmd = vec![format!("cd {path}")];

        if let Some(c) = cfg.cmd_suffix {
            for cc in c {
                // println!("{}", cc.cmd);
                cmd.push(
                    match reg.render_template(cc.cmd.as_str(), &json!({"name": "foo"})) {
                        Ok(s) => s,
                        Err(e) => e.to_string(),
                    },
                );
            }
        }

        // cmd.push(format!("tmux rename-window {name}"));

        cmd.join(";")
    } else {
        "".to_string()
    }
}
