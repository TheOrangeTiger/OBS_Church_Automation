#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::fs;
use eframe::egui;
fn main() {
    let config: Config = match std::fs::read_to_string("config.toml") {
        Ok(s) => toml::from_str::<Config>(&s).unwrap(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::write("config.toml", "# NUMBER MEANINGS\n# 0 = unidentified\n# 1 = credits\n# 2 = regular text\n# 3 = hymn\n# 4 = P: C:\n# 5 = insert empty scene\n# 6 = service name\n# 7 = lords prayer\n# 8 = special music\n# FORMAT\n# case = number\n[cases]").unwrap();
            Config { cases: vec![] }
        }
        Err(e) => panic!("Failed to read config: {}", e),
    };
    env_logger::init();
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    let data = Data {list: bulletin_categorizer(bulletin_reader(), config.cases), save: false, multi_select: false};
    let _ = eframe::run_native(
        "Church OBS Automator",
        options,
        Box::new(|_| {
            Ok(Box::new(data))
        }),
    );
}
struct Data {
    list: Vec<(u32, String)>,
    save: bool,
    multi_select: bool
}
impl eframe::App for Data {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Church Automator");
                ui.label("      Save Contents: ");
                if ui.button("save").clicked() {
                    self.save = true;
                }
                ui.label("        Multi Select: ");
                ui.checkbox(&mut self.multi_select, "");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                for (i, line) in &mut self.list.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let mut display: String = line.1.clone();
                        if line.1.len() > 15 {
                            display = line.1.clone()[..15].to_string();
                        }
                        ui.push_id(i, |ui| {
                            ui.collapsing(display, |ui| {
                                ui.label(line.1.clone());
                            });
                        });
                        egui::ComboBox::from_id_salt(i)
                            .selected_text(format!("{}", line.0))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut line.0, 0, "0 - do nothing");
                                ui.selectable_value(&mut line.0, 1, "1 - credits");
                                ui.selectable_value(&mut line.0, 2, "2 - regular text");
                                ui.selectable_value(&mut line.0, 3, "3 - hymn");
                                ui.selectable_value(&mut line.0, 4, "4 - P: C:");
                                ui.selectable_value(&mut line.0, 5, "5 - empty scene");
                                ui.selectable_value(&mut line.0, 6, "6 - service name");
                                ui.selectable_value(&mut line.0, 7, "7 - lords prayer");
                                ui.selectable_value(&mut line.0, 8, "8 - special music");
                                ui.selectable_value(&mut line.0, 9, "9 - with previous");
                            });
                    });
                }
            });
            if self.save {
                self.save = false;
                save_obs_file(build_livestream(self.list.clone()));
            }
        });
    }
}
#[derive(Serialize)]
#[serde(untagged)]
enum Source {
    Scene {
        name: String,
        enabled: bool,
        id: String,
        settings: Items
    },
    Text {
        name: String,
        id: String,
        settings: TextSettings
    }
}
#[derive(Serialize)]
struct TextSettings {
    text: String,
    align: String,
    color: u32,
    bk_color: u32,
    bk_opacity: u32,
    font: FontSettings
}
#[derive(Serialize)]
struct FontSettings {
    size: u32
}
#[derive(Serialize)]
struct Items {
    items: Vec<TextObj>
}
#[derive(Serialize)]
struct TextObj {
    name: String,
    visible: bool,
    scale_ref: Position,
    pos: Position
}
#[derive(Serialize, Clone, Copy)]
struct Position {
    x: f32,
    y: f32
}
#[derive(Serialize)]
struct Main {
    scene_order: Vec<Name>,
    current_scene: String,
    name: String,
    sources: Vec<Source>
}
#[derive(Serialize)]
struct Name {
    name: String
}
fn init_main(name: &str) -> Main {
    add_textobj(add_scene(add_scene( 
    Main {
        scene_order: vec![],
        current_scene: "Camera".to_string(),
        name: name.to_string(),
        sources: vec![]
    }, "Camera"), "Intro Slide"), "License", "Intro Slide", " Music and Images: OneLicense A - 730010 \nCCLI #3385233\n© Trinity Lutheran Church 2025", 40, Position {x: 25.0, y: 934.0}, 4281983947, 4291523388, 50, "center")
}
fn add_scene(mut main: Main, name: &str) -> Main {
    main.scene_order.push(Name {name: name.to_string()});
    main.sources.push(Source::Scene { name: name.to_string(), enabled: true, id: "scene".to_string(), settings: Items { items: vec![TextObj {name: "Camera".to_string(), visible: true, scale_ref: Position { x: 1920.0, y: 1080.0 }, pos: Position { x: 0.0, y: 0.0 },}] } });
    main
}
fn add_textobj(mut main: Main, name: &str, scene: &str, contents: &str, fontsize: u32, position: Position, text_colour: u32, bg_colour: u32, bg_opacity: u32, align: &str) -> Main {
    main.sources.push(Source::Text { name: name.to_string(), id: "text_gdiplus".to_string(), settings: TextSettings { text: contents.to_string(), align: align.to_string(), font: FontSettings { size: fontsize }, color: text_colour, bk_color: bg_colour, bk_opacity: bg_opacity } });
    for source in main.sources.iter_mut() {
        if let Source::Scene {name: targeted_scene, settings, ..} = source {
            if targeted_scene == scene {
                settings.items.push(TextObj {
                    name: name.to_string(),
                    visible: true,
                    scale_ref: Position { x: 1920.0, y: 1080.0 },
                    pos: position
                });
            }
        }
    }
    main
}
fn bulletin_reader() -> Vec<String> {
    let mut lines = vec![];
    let f = File::open("bulletin.txt").expect("Failed to open file");
    let linestemp = BufReader::new(f);
    for line in linestemp.lines() { lines.push(line.unwrap()); }
    lines
}
// NUMBER MEANINGS
// 0 = unidentified
// 1 = credits
// 2 = regular text
// 3 = hymn
// 4 = P: C:
// 5 = insert empty scene
// 6 = service name
// 7 = lords prayer
// 8 = special music
#[derive(Deserialize)]
struct Config {
    cases: Vec<(u32, String)>
}
fn bulletin_categorizer(bulliten: Vec<String>, cases: Vec<(u32, String)>) -> Vec<(u32, String)> {
    let mut map: Vec<(u32, String)> = vec![];
    let mut bulliten_index = 1;
    map.push((6, bulliten[0].clone()));
    while bulliten_index < bulliten.len() {
        let line = bulliten[bulliten_index].trim().to_string();
        if line != "" {
            map.push((0, line.clone()));
            for case in &cases {
                if line.contains(&case.1) {
                    if case.0 == 10  {
                        if line.starts_with(&case.1) {
                            map.pop();
                            map.push((2, line.clone()));
                            bulliten_index += 1;
                            map.push((9, bulliten[bulliten_index].trim().to_string()));
                        }
                    } else {
                        map.pop();
                        map.push((case.0, line.clone()));
                    }

                }
            }
        }   
        bulliten_index += 1;
    }
    map
}
fn _user_interaction_cli(mut map: Vec<(u32, String)>) {
    let menu_states: Vec<&str> = vec![
        "----------------\n   Main Menu\n----------------\nPress 1 to edit the contents\nPress 2 to save the file\nPress 3 to exit", 
        "----------------\n   Edit Menu\n----------------\nNUMBER MEANINGS\n0 = unidentified\n1 = credits\n2 = regular text\n3 = hymn\n4 = P: C:\n5 = insert empty scene\n6 = service name\n7 = lords prayer\n8 = special music\n9 = with previous"
    ];
    println!("{}", menu_states[0]);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: String = input.trim().to_lowercase();
        if input == "3" {return}
        else if input == "2" {save_obs_file(build_livestream(map.clone()));}
        else if input == "1" {
            println!("{}", menu_states[1]);
            let mut map_index = 0;
            while map_index < map.len() {
                let (k, v) = map.iter().nth(map_index).unwrap();
                println!("Keep, Change, Back or Exit (k/c/b/e)");
                println!("{} {}", k, v);
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input: String = input.trim().to_lowercase();
                if input == "k" {map_index += 1;}
                else if input == "b" {map_index -= 1;}
                else if input == "e" {break;}
                else if input == "c" {
                    println!("What to change the number to?\n0 = unidentified\n1 = credits\n2 = regular text\n3 = hymn\n4 = P: C:\n5 = insert empty scene\n6 = service name\n7 = lords prayer\n8 = special music");
                    loop {
                        let input: u32 = loop {
                            let mut raw = String::new();
                            std::io::stdin().read_line(&mut raw).unwrap();
                            match raw.trim().parse() {
                                Ok(n) => break n,
                                Err(_) => println!("Please enter a valid number")
                            }
                        };
                        if input < 10 {map[map_index].0 = input; break;}
                    }
                    map_index += 1;
                }
            }
        }
        println!("{}", menu_states[0]);
    }
}
// NUMBER MEANINGS
// 0 = unidentified
// 1 = credits
// 2 = regular text
// 3 = hymn
// 4 = P: C:
// 5 = insert empty scene
// 6 = service name
// 7 = lords prayer
// 8 = special music
// 9 = dont insert
fn build_livestream(map: Vec<(u32, String)>) -> Main {
    let mut main = init_main(&map[0].1);
    main = add_textobj(main, "Service Name", "Intro Slide", &format!(" {} \n Trinity Lutheran Church - Edmonton ", map[0].1), 55, Position {x: 0.0, y: 75.0}, 4281983947, 4291523388, 50, "center");
    let mut index = 0;
    let mut back_count: usize = 0;
    let mut fallback_count: usize = 0;
    while index < map.len() {
        if map[index].0 == 2 { main = add_scene(main, &format!("scn_{}", map[index].1)); main = add_textobj(main, &format!("txt_{}", map[index].1), &format!("scn_{}", map[index].1), &wrap_text(&map[index].1, 40), 50, Position {x: 20.0, y: 20.0}, 4278190080, 4294967295, 75, "left"); }
        else if map[index].0 == 3 { main = add_scene(main, &format!("scn_{}", map[index].1)); }
        else if map[index].0 == 5 { if index + 1 >= map.len() || map[index + 1].0 != 5 { main = add_scene(main, &format!("scn_{}", map[index].1)); } }
        else if map[index].0 == 4 || map[index].0 == 1 || map[index].0 == 8 { main = add_scene(main, &format!("scn_{}", map[index].1)); main = add_textobj(main, &format!("txt_{}", map[index].1), &format!("scn_{}", map[index].1), &format!(" {} ", map[index].1), 50, Position {x: 0.0, y: 0.0}, 4278190080, 4294967295, 75, "center"); }
        else if map[index].0 == 7 { main = add_scene(main, &format!("scn_{}", map[index].1)); main = add_textobj(main, &format!("txt_{}", map[index].1), &format!("scn_{}", map[index].1), " Our Father, who art in heaven,\n hallowed be thy Name,\n thy kingdom come,\n thy will be done,\n on earth as it is in heaven.\n Give us this day our daily bread.\n And forgive us our trespasses,\n as we forgive those who trespass against us.\n And lead us not into temptation,\n but deliver us from evil.\n For thine is the kingdom, and the power, and the glory, \n for ever and ever. Amen.", 50, Position {x: 20.0, y: 20.0}, 4278190080, 4294967295, 75, "left"); }
        else if map[index].0 == 9 {
            let mut temp_index = index.clone() - 1;
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
                            settings.text = format!("{}\n{}", settings.text, wrap_text(&map[index].1, 40));
                        } else {
                            settings.text = format!("{}\n{}", settings.text, map[index].1);
                        }
                        break;
                    }
                } else if temp_index == 0 {
                    main = add_scene(main, &format!("scn_{}", map[index].1)); main = add_textobj(main, &format!("txt_{}", map[index].1), &format!("scn_{}", map[index].1), &wrap_text(&map[index].1, 40), 50, Position {x: 20.0, y: 20.0}, 4278190080, 4294967295, 75, "left");
                    fallback_count += 1;
                    break;
                } else { 
                    back_count += 1;
                    temp_index -= 1;
                }
            }
        }
        index += 1;
    }
    println!("back count: {}", back_count);
    println!("fallback count: {}", fallback_count);
    main
}
fn wrap_text(text: &str, width: usize) -> String {
    let ans = text.lines()
        .map(|line| wrap_line(line, width))
        .collect::<Vec<String>>()
        .join("\n");
    ans.lines().map(|line| format!(" {} ", line)).collect::<Vec<String>>().join("\n")
}
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
fn save_obs_file(main: Main) {
    fs::write("obs_file.json", serde_json::to_string_pretty(&main).expect("Failed")).unwrap();
}
