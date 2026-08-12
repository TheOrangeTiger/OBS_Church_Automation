#[cfg_attr(mobile, tauri::mobile_entry_point)]
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tauri_plugin_dialog::DialogExt;
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            init_main,
            build_livestream,
            bulletin_categorizer,
            bulletin_reader,
            save_obs_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Source {
    Scene {
        name: String,
        enabled: bool,
        id: String,
        settings: Items,
    },
    Text {
        name: String,
        id: String,
        settings: TextSettings,
    },
}
#[derive(Serialize, Deserialize)]
struct TextSettings {
    text: String,
    align: String,
    color: u32,
    bk_color: u32,
    bk_opacity: u32,
    font: FontSettings,
}
#[derive(Serialize, Deserialize)]
struct FontSettings {
    size: u32,
}
#[derive(Serialize, Deserialize)]
struct Items {
    items: Vec<TextObj>,
}
#[derive(Serialize, Deserialize)]
struct TextObj {
    name: String,
    visible: bool,
    scale_ref: Position,
    pos: Position,
}
#[derive(Serialize, Deserialize, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Serialize, Deserialize)]
struct Main {
    scene_order: Vec<Name>,
    current_scene: String,
    name: String,
    sources: Vec<Source>,
}
#[derive(Serialize, Deserialize)]
struct Name {
    name: String,
}
#[derive(Deserialize, Serialize)]
struct Config {
    cases: Vec<(u8, String)>,
}
#[tauri::command]
fn get_config() -> Config {
    let config: Config = match std::fs::read_to_string("config.toml") {
        Ok(s) => toml::from_str::<Config>(&s).unwrap_or(Config { cases: vec![] }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::write("config.toml", "# NUMBER MEANINGS\n# 0 = unidentified\n# 1 = credits\n# 2 = regular text\n# 3 = hymn\n# 4 = P: C:\n# 5 = insert empty scene\n# 6 = service name\n# 7 = N/A\n# 8 = special music\n# 9 = with previous\n# 10 = this line 2 next line 9\n# FORMAT\ncases = [\n\t[2, \"hello\"],\n]\ncases = []").unwrap();
            Config { cases: vec![] }
        }
        Err(e) => panic!("Failed to read config: {}", e),
    };
    config
}
#[tauri::command]
fn init_main(name: &str) -> Main {
    add_textobj(
        add_scene(
            add_scene(
                Main {
                    scene_order: vec![],
                    current_scene: "Camera".to_string(),
                    name: name.to_string(),
                    sources: vec![],
                },
                "Camera",
            ),
            "Intro Slide",
        ),
        "License",
        "Intro Slide",
        " Music and Images: OneLicense A - 730010 \nCCLI #3385233\n© Trinity Lutheran Church 2025",
        40,
        Position { x: 25.0, y: 934.0 },
        4281983947,
        4291523388,
        50,
        "center",
    )
}
#[tauri::command]
fn add_scene(mut main: Main, name: &str) -> Main {
    main.scene_order.push(Name {
        name: name.to_string(),
    });
    main.sources.push(Source::Scene {
        name: name.to_string(),
        enabled: true,
        id: "scene".to_string(),
        settings: Items {
            items: vec![TextObj {
                name: "Camera".to_string(),
                visible: true,
                scale_ref: Position {
                    x: 1920.0,
                    y: 1080.0,
                },
                pos: Position { x: 0.0, y: 0.0 },
            }],
        },
    });
    main
}
#[tauri::command]
fn add_textobj(
    mut main: Main,
    name: &str,
    scene: &str,
    contents: &str,
    fontsize: u32,
    position: Position,
    text_colour: u32,
    bg_colour: u32,
    bg_opacity: u32,
    align: &str,
) -> Main {
    main.sources.push(Source::Text {
        name: name.to_string(),
        id: "text_gdiplus".to_string(),
        settings: TextSettings {
            text: contents.to_string(),
            align: align.to_string(),
            font: FontSettings { size: fontsize },
            color: text_colour,
            bk_color: bg_colour,
            bk_opacity: bg_opacity,
        },
    });
    for source in main.sources.iter_mut() {
        if let Source::Scene {
            name: targeted_scene,
            settings,
            ..
        } = source
        {
            if targeted_scene == scene {
                settings.items.push(TextObj {
                    name: name.to_string(),
                    visible: true,
                    scale_ref: Position {
                        x: 1920.0,
                        y: 1080.0,
                    },
                    pos: position,
                });
            }
        }
    }
    main
}
#[tauri::command]
fn bulletin_reader(app: tauri::AppHandle) -> Result<Vec<String>, u8> {
    let mut lines = vec![];
    let file_path = app
        .dialog()
        .file()
        .blocking_pick_file()
        .map(|x| x.to_string());
    if file_path.is_none() {
        Err(0)
    } else {
        let file_path = file_path.unwrap();
        let f = File::open(file_path).expect("Failed to open file");
        let linestemp = BufReader::new(f);
        for line in linestemp.lines() {
            lines.push(line.unwrap());
        }
        Ok(lines)
    }
}
#[tauri::command]
fn bulletin_categorizer(bulliten: Vec<String>, config: Config) -> Vec<(u8, String)> {
    let cases = config.cases;
    let mut map: Vec<(u8, String)> = vec![];
    let mut bulliten_index = 1;
    map.push((6, bulliten[0].clone()));
    while bulliten_index < bulliten.len() {
        let line = bulliten[bulliten_index].trim().to_string();
        if line.is_empty() {
        } else if line.contains("Lord’s Prayer") {
            map.push((2, line));
            map.push((
                9,
                "Our Father, who art in heaven,
                hallowed be thy Name,
                thy kingdom come,
                thy will be done,
                on earth as it is in heaven.
                Give us this day our daily bread.
                And forgive us our trespasses,
                as we forgive those
                who trespass against us.
                And lead us not into temptation,
                but deliver us from evil.
                For thine is the kingdom,
                and the power, and the glory,
                for ever and ever.
                Amen."
                    .to_string(),
            ));
        } else if line.contains("Apostles’ Creed") {
            map.push((2, line));
            map.push((
                9,
                "I believe in God,
                the Father almighty,
                Creator of heaven and earth,
                and in Jesus Christ, his only Son, our Lord,
                who was conceived by the Holy Spirit,
                born of the Virgin Mary,
                suffered under Pontius Pilate,
                was crucified, died and was buried;
                he descended into hell;
                on the third day he rose again from the dead;
                he ascended into heaven,
                and is seated at the right hand of God the Father almighty;
                from there he will come to judge the living and the dead.
                I believe in the Holy Spirit,
                the holy catholic Church,
                the communion of saints,
                the forgiveness of sins,
                the resurrection of the body,
                and life everlasting.
                Amen."
                    .to_string(),
            ));
        } else if line.contains("Nicene Creed") {
            map.push((2, line));
            map.push((
                9,
                "We believe in one God,
                the Father almighty,
                maker of heaven and earth,
                of all things visible and invisible.
                And in one Lord Jesus Christ,
                the only Son of God,
                begotten from the Father before all ages,
                God from God,
                Light from Light,
                true God from true God,
                begotten, not made;
                of the same essence as the Father.
                Through him all things were made.
                For us and for our salvation
                he came down from heaven;
                he became incarnate by the Holy Spirit and the virgin Mary,
                and was made human.
                He was crucified for us under Pontius Pilate;
                he suffered and was buried.
                The third day he rose again, according to the Scriptures.
                He ascended to heaven
                and is seated at the right hand of the Father.
                He will come again with glory
                to judge the living and the dead.
                His kingdom will never end.
                And we believe in the Holy Spirit,
                the Lord, the giver of life.
                He proceeds from the Father and the Son,
                and with the Father and the Son is worshiped and glorified.
                He spoke through the prophets.
                We believe in one holy catholic and apostolic church.
                We affirm one baptism for the forgiveness of sins.
                We look forward to the resurrection of the dead,
                and to life in the world to come. Amen."
                    .to_string(),
            ));
        } else {
            map.push((0, line.clone()));
            for case in &cases {
                if line.contains(&case.1) {
                    if case.0 == 10 {
                        if line.starts_with(&case.1) {
                            map.pop();
                            map.push((2, line));
                            bulliten_index += 1;
                            map.push((9, bulliten[bulliten_index].trim().to_string()));
                        }
                    } else {
                        map.pop();
                        map.push((case.0, line));
                    }
                    break;
                }
            }
        }
        bulliten_index += 1;
    }
    map
}
#[tauri::command]
fn build_livestream(map: Vec<(u8, String)>) -> Main {
    let mut main = init_main(&map[0].1);
    main = add_textobj(
        main,
        "Service Name",
        "Intro Slide",
        &format!(" {} \n Trinity Lutheran Church - Edmonton ", map[0].1),
        55,
        Position { x: 0.0, y: 75.0 },
        4281983947,
        4291523388,
        50,
        "center",
    );
    let mut index = 0;
    while index < map.len() {
        if map[index].0 == 2 {
            main = add_scene(main, &format!("scn_{}", map[index].1));
            main = add_textobj(
                main,
                &format!("txt_{}", map[index].1),
                &format!("scn_{}", map[index].1),
                &wrap_text(&map[index].1, 40),
                50,
                Position { x: 20.0, y: 20.0 },
                4278190080,
                4294967295,
                75,
                "left",
            );
        } else if map[index].0 == 3 {
            main = add_scene(main, &format!("scn_{}", map[index].1));
        } else if map[index].0 == 5 {
            if index + 1 >= map.len() || map[index + 1].0 != 5 {
                main = add_scene(main, &format!("scn_{}", map[index].1));
            }
        } else if map[index].0 == 4 || map[index].0 == 1 || map[index].0 == 8 {
            main = add_scene(main, &format!("scn_{}", map[index].1));
            main = add_textobj(
                main,
                &format!("txt_{}", map[index].1),
                &format!("scn_{}", map[index].1),
                &format!(" {} ", map[index].1),
                50,
                Position { x: 0.0, y: 0.0 },
                4278190080,
                4294967295,
                75,
                "center",
            );
        }
        // else if map[index].0 == 7 { main = add_scene(main, &format!("scn_{}", map[index].1)); main = add_textobj(main, &format!("txt_{}", map[index].1), &format!("scn_{}", map[index].1), " Our Father, who art in heaven,\n hallowed be thy Name,\n thy kingdom come,\n thy will be done,\n on earth as it is in heaven.\n Give us this day our daily bread.\n And forgive us our trespasses,\n as we forgive those who trespass against us.\n And lead us not into temptation,\n but deliver us from evil.\n For thine is the kingdom, and the power, and the glory, \n for ever and ever. Amen.", 50, Position {x: 20.0, y: 20.0}, 4278190080, 4294967295, 75, "left"); }
        else if map[index].0 == 9 {
            let mut temp_index = index - 1;
            loop {
                if map[temp_index].0 == 1 || map[temp_index].0 == 2 || map[temp_index].0 == 4 {
                    if let Some(Source::Text { settings, .. }) = main.sources.iter_mut().find(|x| {
                        if let Source::Text { name, .. } = x {
                            name == &format!("txt_{}", map[temp_index].1)
                        } else {
                            false
                        }
                    }) {
                        if map[temp_index].0 == 2 {
                            settings.text =
                                format!("{}\n{}", settings.text, wrap_text(&map[index].1, 40));
                        } else {
                            settings.text = format!("{}\n{}", settings.text, map[index].1);
                        }
                        break;
                    }
                } else if temp_index == 0 {
                    main = add_scene(main, &format!("scn_{}", map[index].1));
                    main = add_textobj(
                        main,
                        &format!("txt_{}", map[index].1),
                        &format!("scn_{}", map[index].1),
                        &wrap_text(&map[index].1, 40),
                        50,
                        Position { x: 20.0, y: 20.0 },
                        4278190080,
                        4294967295,
                        75,
                        "left",
                    );
                    break;
                } else {
                    temp_index -= 1;
                }
            }
        }
        index += 1;
    }
    main
}
#[tauri::command]
fn wrap_text(text: &str, width: usize) -> String {
    let ans = text
        .lines()
        .map(|line| wrap_line(line, width))
        .collect::<Vec<String>>()
        .join("\n");
    ans.lines()
        .map(|line| format!(" {} ", line))
        .collect::<Vec<String>>()
        .join("\n")
}
#[tauri::command]
fn wrap_line(text: &str, width: usize) -> String {
    let mut result = String::new();
    let mut line_len = 0;
    for word in text.split_whitespace() {
        if line_len + word.len() > width && line_len > 0 {
            result.push('\n');
            line_len = 0;
        } else if line_len > 0 {
            result.push(' ');
            line_len += 1;
        }
        result.push_str(word);
        line_len += word.len();
    }
    result
}
#[tauri::command]
fn save_obs_file(main: Main) {
    fs::write(
        format!("{}.json", main.name),
        serde_json::to_string_pretty(&main).expect("Failed"),
    )
    .unwrap();
}
use figlet_rs::FIGlet;
use owo_colors::OwoColorize;
// Yellow bold w \t for main messages
// Blue for choices
// Red bold for errors or unexpected occuences
use std::fs::read_to_string;
use std::io;
pub fn cli() {
    let font = FIGlet::standard().unwrap();
    println!(
        "{}{}{}{}",
        font.convert("OBS").unwrap().yellow().bold(),
        font.convert("Church").unwrap().yellow().bold(),
        font.convert("Automation").unwrap().yellow().bold(),
        font.convert(": ]").unwrap().yellow().bold()
    );
    'big_loop: loop {
        println!(
            "{}\n{}\n{}\n{}",
            "\tWould you like to:".yellow().bold(),
            "(1) Generate a json based on a txt file".blue(),
            "(2) Edit an existing json".blue(),
            "(3) Exit the program".blue()
        );
        let input = grab_input(Some(vec!["1", "2", "3"]));
        match input.as_str() {
            "1" => {
                let txt = loop {
                    println!("{}", "\tPlease enter the path to the txt".yellow().bold());
                    let path = grab_input(None);
                    if !path.ends_with(".txt") {
                        println!("{}", "Must be a txt file".red().bold());
                        continue;
                    }
                    let txt = read_to_string(&path);
                    let Ok(s) = txt else {
                        println!("{}{}", "Err: ".red().bold(), txt.unwrap_err().red().bold());
                        continue;
                    };
                    break s.trim().to_string();
                };
                let mut line: usize = 0;
                let mut sifted = bulletin_categorizer(
                    txt.lines().map(|x| x.to_string()).collect(),
                    get_config(),
                );
                loop {
                    println!(
                        "{}{}",
                        "\tActions:\n".yellow().bold(),
                        "(0) = unidentified\n(1) = credits\n(2) = regular text\n(3) = hymn\n(4) = P: C:\n(5) = insert empty scene\n(6) = service name\n(7) = N/A\n(8) = special music\n(9) = with previous\n(w) = write and exit\n() = do nothing\n(b) = go back".blue()
                    );
                    println!(
                        "{}{}",
                        sifted[line].0.bright_green(),
                        "\tLine:".yellow().bold()
                    );
                    println!("{}", wrap_line(&sifted[line].1, 75).green());
                    let action = grab_input(Some(vec![
                        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "w", "", "b",
                    ]));
                    if action.as_str() == "w" {
                        break;
                    } else if action.is_empty() {
                        if line <= sifted.len() {
                            line += 1;
                        }
                        continue;
                    } else if action.as_str() == "b" {
                        if line != 0 {
                            line -= 1;
                        }
                        continue;
                    }
                    sifted[line].0 = action.parse().unwrap_or(0);
                }
                save_obs_file(build_livestream(sifted));
            }
            "2" => {
                println!("{}", "Editing json not yet supported".red().bold());
                continue;
                loop {
                    println!("{}", "\tPlease enter the path to the json".yellow().bold());
                    let path = grab_input(None);
                    if !path.ends_with(".json") {
                        println!("{}", "Must be a json file".red().bold());
                        continue;
                    }
                    let txt = read_to_string(&path);
                    let Ok(s) = txt else {
                        println!("{}{}", "Err: ".red().bold(), txt.unwrap_err().red().bold());
                        continue;
                    };
                    break;
                }
            }
            "3" => {
                break 'big_loop;
            }
            _ => unreachable!(),
        }
    }
}
fn grab_input(allowable_cases: Option<Vec<&str>>) -> String {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(c) = &allowable_cases {
                    if !c.contains(&&input.trim().to_lowercase().as_str()) {
                        println!("{}", "Please provide a valid input".red().bold());
                        continue;
                    }
                }
            }
            Err(e) => {
                println!(
                    "{}{e}{}",
                    "Encountered error: ".red().bold(),
                    "\nPlease try again".red().bold()
                );
                continue;
            }
        }
        return input.trim().to_string();
    }
}
