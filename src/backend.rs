use serde_derive::{Deserialize, Serialize};
use std::fs;
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
pub struct Main {
    scene_order: Vec<Name>,
    current_scene: String,
    name: String,
    sources: Vec<Source>,
}
impl Main {
    fn new(name: &str) -> Self {
        let mut main = Main {
            scene_order: vec![],
            current_scene: "Camera".to_string(),
            name: name.to_string(),
            sources: vec![],
        };
        main.add_scene("Camera");
        main.add_scene("Intro Slide");
        main.add_text_obj("License",
            "Intro Slide",
            " Music and Images: OneLicense A - 730010 \nCCLI #3385233\n© Trinity Lutheran Church 2025",
            40,
            Position { x: 25.0, y: 934.0 },
            4281983947,
            4291523388,
            50,
            "center",);
        main
    }
    fn add_text_obj(
        &mut self,
        name: &str,
        scene: &str,
        contents: &str,
        fontsize: u32,
        position: Position,
        text_colour: u32,
        bg_colour: u32,
        bg_opacity: u32,
        align: &str,
    ) {
        self.sources.push(Source::Text {
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
        for source in self.sources.iter_mut() {
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
    }
    fn add_scene(&mut self, name: &str) {
        self.scene_order.push(Name {
            name: name.to_string(),
        });
        self.sources.push(Source::Scene {
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
    }
}
#[derive(Serialize, Deserialize)]
struct Name {
    name: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    cases: Vec<(u8, String)>,
}
pub fn get_config() -> Config {
    let config: Config = match std::fs::read_to_string("config.toml") {
        Ok(s) => toml::from_str::<Config>(&s).unwrap_or(Config { cases: vec![] }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let _ = fs::write("config.toml", "# NUMBER MEANINGS\n# 0 = unidentified\n# 1 = credits\n# 2 = readings\n# 3 = hymn\n# 4 = P: C:\n# 5 = insert empty scene\n# 6 = service name\n# 7 = N/A\n# 8 = special music\n# 9 = with previous\n# 10 = this line 2 next line 9\n# FORMAT\ncases = [\n\t[2, \"hello\"],\n]\ncases = []");
            Config { cases: vec![] }
        }
        Err(_) => Config { cases: vec![] },
    };
    config
}
pub fn get_help() {
    if let Err(e) =
        open::that("https://github.com/TheOrangeTiger/OBS_Church_Automation/blob/main/README.md")
    {
        eprintln!("Failed to open help link: {e}");
    }
}
pub fn bulletin_categorizer(bulliten: Vec<String>, config: Config) -> Vec<(u8, String)> {
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
pub fn build_livestream(map: Vec<(u8, String)>) -> Main {
    let name = match map.iter().find(|(k, _)| *k == 6).map(|(_, v)| v) {
        Some(x) => x.to_string(),
        None => map[0].1.clone(),
    };
    let mut main = Main::new(name.as_str());
    main.add_text_obj(
        "Service Name",
        "Intro Slide",
        &format!(" {} \n Trinity Lutheran Church - Edmonton ", name),
        55,
        Position { x: 0.0, y: 75.0 },
        4281983947,
        4291523388,
        50,
        "center",
    );
    let mut index = 0;
    while index < map.len() {
        // 0 and 7 are skipped
        if map[index].0 == 2 {
            main.add_scene(&format!("scn_{}", map[index].1));
            main.add_text_obj(
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
            main.add_scene(&format!("scn_{}", map[index].1));
        } else if map[index].0 == 5 {
            if index + 1 >= map.len() || map[index + 1].0 != 5 {
                main.add_scene(&format!("scn_{}", map[index].1));
            }
        } else if map[index].0 == 4 || map[index].0 == 1 || map[index].0 == 8 {
            main.add_scene(&format!("scn_{}", map[index].1));
            main.add_text_obj(
                &format!("txt_{}", map[index].1),
                &format!("scn_{}", map[index].1),
                &wrap_text(&map[index].1, 100),
                50,
                Position { x: 0.0, y: 0.0 },
                4278190080,
                4294967295,
                75,
                "center",
            );
        } else if map[index].0 == 9 {
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
                    main.add_scene(&format!("scn_{}", map[index].1));
                    main.add_text_obj(
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
pub fn wrap_line(text: &str, width: usize) -> String {
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
pub fn save_obs_file(main: Main) {
    let _ = fs::write(
        format!("{}.json", main.name),
        serde_json::to_string_pretty(&main).unwrap_or("Failed to save JSON".to_string()),
    );
}
