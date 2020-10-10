//#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

extern crate lib_sens as sens;

use std::io;
use web_view::*;
use structopt::StructOpt;
use sens::games_enum::SupportedGames;
use sens::profile_manager::Profiles;

#[derive(StructOpt)]
struct Cli {
    ///sens commands: <ls> to list profiles and steam paths; <set> to set a specific sensitivity value; <eq> to equalize a profile to one game's sens value; <add> to add a profile or steam folder; <rm> to remove a steam folder or profile; <sw> to switch the active profile; <show> to show the values in a profile; <name> to change the name of a profile; <load> to load the sensitivity values from game configuration files
    #[structopt(default_value = "gui")]
    command: String,
    ///Flag to deal with steam folders, not profiles
    #[structopt(short, long)]
    steam: bool,
    ///Path of the steam folder
    #[structopt(short = "p", long = "path", default_value = "default_value")]
    path: String,
    ///Name of the profile
    #[structopt(short = "n", long = "name", default_value = "default_value")]
    name: String,
    ///Float Value that you are setting
    #[structopt(short = "v", long = "value", default_value = "0.0")]
    value: f64,
    ///Index of profile or steam folder
    #[structopt(short = "i", long = "index", default_value = "0")]
    index: i32,
    ///The name of the game which you are modifying the sens value of
    #[structopt(short = "g", long = "game", default_value = "CSGO")]
    game: String,
}

fn main() {
    let args = Cli::from_args();

    match args.command.as_str() {
        "ls" => println!("{}", Profiles::fs_load_profiles().unwrap().to_string(args.steam)),
        "set" => set(args.game, args.value, args.index).unwrap(),
        "eq" => eq(args.game, args.index).unwrap(),
        "add" => {
            if args.steam {
                add_steam(args.path).unwrap()
            } else {
                add().unwrap()
            }
        }
        "rm" => {
            if args.steam {
                rm_steam(args.index).unwrap()
            } else {
                rm(args.index).unwrap()
            }
        }
        "sw" => switch(args.index).unwrap(),
        "show" => println!(
            "{}",
            Profiles::fs_load_profiles()
                .unwrap()
                .show_profile(args.index)
        ),
        "name" => rename(args.index, args.name).unwrap(),
        "load" => load(args.index).unwrap(),
        _ => gui(),
    }
}

fn gui() {
    let html = format!(
        r#"
        {doctype}
        <html>
        <head>
        {head}
        {main_style}
        {bulma}
        </head>
        <body onload='update()'>
        {body}
        {scripts}
        </body>
        </html>"#,
        doctype = include_str!("html/doctype.html"),
        head = include_str!("html/head.html"),
        body = include_str!("html/body.html"),
        main_style = inline_style(include_str!("css/index.css")),
        bulma = inline_style(include_str!("css/bulma.min.css")),
        scripts = inline_script(include_str!("js/index.js")),
        );

    let mut webview = web_view::builder()
    .title("sens")
    .content(Content::Html(html))
    .size(800, 450)
    .resizable(true)
    .debug(true)
    .user_data("hi")
    .invoke_handler(|webview, arg| {
        if serde_json::from_str::<Cmd>(arg).is_ok() {
            println!(
                "Recieved Command: {:?}",
                serde_json::from_str::<Cmd>(arg).unwrap()
                );
            serde_json::from_str::<Cmd>(arg).unwrap().process();
        } else if serde_json::from_str::<Profiles>(arg).is_ok() {
            println!(
                "Recieved Profiles: {}",
                serde_json::from_str::<Profiles>(arg)
                .unwrap()
                .to_string(false)
                );
            serde_json::from_str::<Profiles>(arg)
            .unwrap()
            .save_json()
            .unwrap();
        } else {
            match arg {
                "get" => {
                    println!(
                        "Sent to client: {}",
                        &format!(
                            "catchIt({})",
                            serde_json::to_string(&Profiles::fs_load_profiles().unwrap())
                            .unwrap()
                            )
                        );
                    webview
                    .eval(&format!(
                        "catchObj({})",
                        serde_json::to_string(&Profiles::fs_load_profiles().unwrap())
                        .unwrap()
                        .to_string()
                        ))
                    .unwrap()
                }
                _ => println!("Unimplemented Invoke Handler\n\nObject:\n\n{:?}", serde_json::from_str::<Cmd>(arg)),
            }
        }

        Ok(())
    })
    .build()
    .unwrap();

    webview.set_color((156, 39, 176));

    let _res = webview.run().unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

#[derive(Debug, Serialize, Deserialize)]
struct Cmd {
    command: String,
    value: f64,
    string_value: String,
    steam: bool,
    index: i32,
}

impl Cmd {
    /*fn new() -> Self {
        Self {
            command: "".to_string(),
            value: 1.0,
            string_value: "".to_string(),
            steam: false,
            index: 0,
        }
    }*/

    fn process(&mut self) {
        println!("processing");
        match self.command.as_str() {
            "set" => set(self.string_value.clone(), self.value, self.index).unwrap(),
            "eq" => eq(self.string_value.clone(), self.index).unwrap(),
            "add" => {
                if self.steam {
                    println!("adding steam path");
                    add_steam(self.string_value.clone()).unwrap()
                } else {
                    add().unwrap()
                }
            }
            "rm" => {
                if self.steam {
                    rm_steam(self.index).unwrap()
                } else {
                    rm(self.index).unwrap()
                }
            }
            "sw" => switch(self.index).unwrap(),
            "name" => rename(self.index, self.string_value.clone()).unwrap(),
            "load" => load(self.index).unwrap(),
            _ => println!("Command Not Found"),
        }
    }
}

fn set(game: String, sens: f64, profile_index: i32) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.set_game_sens_in_profile(SupportedGames::from_str(game.as_str()), sens, profile_index);
    x.save_json()?;
    Ok(())
}

fn eq(game: String, profile_index: i32) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.equalize_profile_at_index(SupportedGames::from_str(game.as_str()), profile_index);
    x.save_json()?;
    Ok(())
}

fn add() -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.add_profile();
    x.save_json()?;
    Ok(())
}

fn add_steam(path: String) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.append_steam_folder(path);
    x.save_json()?;
    Ok(())
}

fn rm(index: i32) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.remove_profile(index);
    x.save_json()?;
    Ok(())
}

fn rm_steam(index: i32) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.remove_steam_folder_at_index(index);
    x.save_json()?;
    Ok(())
}

fn rename(index: i32, name: String) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.change_name_at_index(index, name);
    x.save_json()?;
    Ok(())
}

fn load(index: i32) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.fs_read_all_game_sens_at_index(index)?;
    x.save_json()?;
    Ok(())
}

fn switch(index: i32) -> Result<(), io::Error> {
    let mut x = Profiles::fs_load_profiles().unwrap();
    x.set_paths()?;
    x.switch_profile(index)?;
    Ok(())
}
