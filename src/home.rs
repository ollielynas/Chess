use crate::DEFAULT_GAME_STATE;
use crate::ability::*;
use crate::game_data::*;
use crate::key_map::*;
// use crate::particles_fnc::*;
use crate::GLOBAL_VERSION;
// use ::rand::prelude::*;
use macroquad::prelude::*;
use savefile::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use crate::death::*;
use ::rand::prelude::*;

fn key_as_string(key: KeyCode) -> String {
    format!("{:?}", key)
}
fn string_to_key(key: &String) -> KeyCode {
    for i in KEYS_TEXT {
        if &key_as_string(i) == key {
            return i as KeyCode;
        }
    }
    return KeyCode::Unknown;
}
#[derive(Savefile, PartialEq)]
pub enum Screen {
    Game,
    Home,
    Settings,
    Keybinds,
    Textures,
    Death,
    AbilitiesScreen,
}

#[derive(Debug, Clone)]
pub struct UserData {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub ability_key: [KeyCode; 5],
    pub abilities: [Abilities; 5],
    pub texture: String,
    pub high_score: f32,
    pub high_round: f32,
    pub help_bubbles: bool
}

#[derive(Savefile)]
struct OtherUserData {
    abilities: [Abilities; 5],
    texture: String,
    high_score: f32,
    high_round: f32,
    help_bubbles: bool
}

impl UserData {
    fn keybinds_from_vec(&mut self, v: Vec<[String; 2]>) {
        for i in v {
            match i[0].as_str() {
                "left" => {
                    self.left = string_to_key(&i[1]);
                }
                "right" => {
                    self.right = string_to_key(&i[1]);
                }
                "up" => {
                    self.up = string_to_key(&i[1]);
                }
                "down" => self.down = string_to_key(&i[1]),
                "ability_key" => {
                    self.ability_key = [
                        string_to_key(
                            &i[1].split(", ").collect::<Vec<&str>>()[0]
                                .to_owned()
                                .replace('[', ""),
                        ),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[1].to_owned()),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[2].to_owned()),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[3].to_owned()),
                        string_to_key(
                            &i[1].split(", ").collect::<Vec<&str>>()[4]
                                .to_owned()
                                .replace(']', ""),
                        ),
                    ];
                }
                _ => {}
            }
        }
    }

    fn keybinds_to_vec(&self) -> Vec<[String; 2]> {
        let ab_keys = format!("{:?}", self.ability_key);
        vec![
            ["left".to_owned(), key_as_string(self.left)],
            ["right".to_owned(), key_as_string(self.right)],
            ["up".to_owned(), key_as_string(self.up)],
            ["down".to_owned(), key_as_string(self.down)],
            ["ability_key".to_owned(), ab_keys],
        ]
    }

    pub fn save(&self) {
        let keys = self.keybinds_to_vec();
        let other = OtherUserData {
            texture: self.texture.clone(),
            abilities: self.abilities,
            high_round: self.high_round,
            high_score: self.high_score,
            help_bubbles: self.help_bubbles
        };

        save_file("keybinds.bin", GLOBAL_VERSION, &keys).unwrap();
        save_file("user_data.bin", GLOBAL_VERSION, &other).unwrap();
    }

    pub fn load(&mut self) {
        self.keybinds_from_vec(match savefile::load_file("keybinds.bin", GLOBAL_VERSION) {
            Ok(e) => e,
            Err(_) => {
                self.save();
                vec![]
            }
        });

        let other: OtherUserData;
        match savefile::load_file("user_data.bin", GLOBAL_VERSION) {
            Ok(e) => {
                other = e;
                self.abilities = other.abilities;
                self.texture = other.texture;
                self.high_round = other.high_round;
                self.high_score = other.high_score;
            }
            Err(e) => {
                println!("Error: {}", e);
                self.save();
            }
        }
    }
}

pub fn new_game() -> GameData {
    let mut data = DEFAULT_GAME_STATE;
    data.alive = true;
    data
}

fn select_ability(data: &mut GameData, user: &mut UserData, em: f32) {
    if is_key_pressed(KeyCode::Escape) {
        data.select_ability.open = false;
        data.screen = Screen::Home;
    }

    let mut ability_list: Vec<Abilities> = Abilities::iter().collect();
    ability_list.remove(0);
    let total_number_of_pages = (ability_list.len() as f32 - 1.0) / 5.0;
    if data.select_ability.page * 5 > ability_list.len() {
        data.select_ability.page = (ability_list.len() as f32 / 5.0).floor() as usize;
    }

    for _ in 0..data.select_ability.page * 5 {
        ability_list.remove(0);
    }

    while ability_list.len() > 5 {
        ability_list.remove(ability_list.len() - 1);
    }

    draw_text("Abilities", 2.0 * em, 2.0 * em as f32, em * 1.6, DARKGRAY);
    draw_text(
        &format!("  {}/{}", data.select_ability.page + 1, total_number_of_pages.ceil()),
        2.0 * em,
        20.5 * em as f32,
        em,
        DARKGRAY,
    );

    let mouse_y = mouse_position().1 / em;
    let mouse_x = mouse_position().0 / em;

    draw_rectangle(
        1.0 * em,
        1.0 * em,
        em * 11.0,
        em * 30.0,
        Color::from_rgba(10, 10, 10, 255),
    );

    draw_rectangle(
        2.0 * em,
        12.0 * em,
        em * 4.75,
        em * 1.1,
        Color::from_rgba(250, 10, 10, 255),
    );

    
    draw_text("Abilities", 2.2 * em, 2.7 * em, em * 2.0, WHITE);
    
    if mouse_x > 2.0 && mouse_x < 6.75 && mouse_y > 12.0 && mouse_y < 13.1 {
        draw_text("Start Game", 2.2 * em, 12.8 * em, em, ORANGE);

    if is_mouse_button_pressed(MouseButton::Left) {
            data.sounds.push(("click".to_owned(), 0.0));
                        if !data.alive {
                            *data = new_game();
            }
            data.screen = Screen::Game;
        }
    }else {
        draw_text("Start Game", 2.2 * em, 12.8 * em, em, WHITE);
    }

    for f in 0..5 {
        let i = f as f32 * 1.4;
        draw_rectangle(
            2.0 * em,
            (4.4 + i) * em,
            em * 9.5,
            em * 1.1,
            Color::from_rgba(5, 5, 5, 255),
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_x > 2.0 && mouse_x < 11.5 && mouse_y > (4.4 + i) && mouse_y < (4.4 + i) + 1.1 {
                data.select_ability.slot = f;
                data.sounds.push(("click".to_owned(), 0.0));
            }
        }

        if data.select_ability.slot == f {
            draw_rectangle(
                2.0 * em,
                (4.4 + i) * em,
                em * 9.5,
                em * 1.1,
                Color::from_rgba(35, 35, 215, 215),
            );
        }

        draw_text(
            &format!("{:?}", user.ability_key[f]),
            2.5 * em,
            (i + 5.3) * em,
            em * 0.4,
            GRAY,
        );

        draw_text(
            &format!("{:?}", metadata(user.abilities[f]).cost),
            10.5 * em,
            (i + 5.1) * em,
            em * 0.8,
            GRAY,
        );

        draw_text(
            &metadata(user.abilities[f]).name,
            2.1  * em,
            (i + 5.0) * em,
            em * 0.8,
            WHITE,
        );
    }

    // nav arrows
    for i in 0..=total_number_of_pages as usize {
        draw_rectangle(
            12.0 * em + i as f32 * em * 2.0,
            match i {
                _ if  mouse_y * em > screen_height() - 1.0*em && (mouse_x - 12.0) >= (i * 2) as f32 && (mouse_x - 12.0) -1.0 <= (i * 2) as f32 => screen_height() - 0.5*em,
                _ => screen_height() - 0.25*em,
            },
            
            em,
            match i {
                _ if mouse_y * em > screen_height() - 1.0*em && (mouse_x - 12.0) >= (i * 2) as f32 && (mouse_x - 12.0) -1.0 <= (i * 2) as f32 => em/2.5,
                _ => em/5.0
            },
            Color { r:
                match i {
                _ if data.select_ability.page == i => 1.0,
                _ => 0.3
                },
            g: 0.3, b: 0.3, a: 1.0 }
        );

        if mouse_y * em > screen_height() - 1.0*em && (mouse_x - 12.0) >= (i * 2) as f32 && (mouse_x - 12.0) -1.0 <= (i * 2) as f32 && is_mouse_button_pressed(MouseButton::Left) {
            data.select_ability.page = i;
        }

        
    }

    // draw ability text
    for i in 0..ability_list.len() {
        let mut color = GRAY;
        let mut line_one: Vec<char> = metadata(ability_list[i]).description.chars().collect();
        let mut line_two = "".to_owned();
        let max_line_length = 60;
        if line_one.len() > max_line_length {
            line_two = (0..line_one.len() - max_line_length)
                .map(|e| line_one[e + max_line_length].to_string())
                .collect::<Vec<String>>()
                .join("");
            line_one = (0..max_line_length)
                .map(|e| line_one[e])
                .collect::<Vec<char>>();
            loop {
                if max_line_length - line_one.len() > 7 {
                    break;
                }
                if line_one[line_one.len() - 1] != ' ' {
                    line_two = line_one[line_one.len() - 1].to_string() + &line_two;
                    line_one.pop();
                } else {
                    break;
                }
            }
        }

        // draw descriptions and names of abilities
        if mouse_y > ((i as f32 * 2.5) + 1.0) * 1.5
            && mouse_y < ((i as f32 * 2.5) + 1.0) * 1.5 + 2.5
            && mouse_x > 13.0
        {
            color = LIGHTGRAY;
            if is_mouse_button_pressed(MouseButton::Left) {
                user.abilities[data.select_ability.slot] = ability_list[i];
                data.sounds.push(("click".to_owned(), 0.0));
                user.save();
            }
        }
        draw_text(
            &metadata(ability_list[i]).name,
            13.0 * em,
            ((i as f32 * 2.5) * em + 2.0 * em) * 1.5,
            em * 1.2,
            color,
        );
        draw_text(
            &format!("Cost: {}", metadata(ability_list[i]).cost),
            screen_width() - 4.0 * em,
            ((i as f32 * 2.5) * em + 2.0 * em) * 1.5,
            em * 0.8,
            color,
        );

        let length_of_effect = match metadata(ability_list[i]).duration {
                Some(a) => format!("{} moves", a),
                None => "Instant".to_owned(),
        };


        draw_text(
            &length_of_effect,
            screen_width() - 4.0 * em,
            ((i as f32 * 2.5) * em + 2.3 * em) * 1.5,
            em * 0.8,
            color,
        );
        draw_text(
            &(&line_one.clone().into_iter().collect::<String>()),
            13.0 * em,
            ((i as f32 * 2.5) * em + 2.8 * em) * 1.5,
            em * 0.8,
            color,
        );
        if line_two != "".to_owned() {
            draw_text(
                &line_two.clone(),
                13.0 * em,
                ((i as f32 * 2.5) * em + 3.2 * em) * 1.5,
                em * 0.8,
                color,
            );
        }
    }
}

/*oooo    oooo                        .o8        o8o                    .o8           
`888   .8P'                        "888        `"'                   "888           
-888  d8'     .ooooo.  oooo    ooo  888oooo.  oooo  ooo. .oo.    .oooo888   .oooo.o 
-88888[      d88' `88b  `88.  .8'   d88' `88b `888  `888P"Y88b  d88' `888  d88(  "8 
-888`88b.    888ooo888   `88..8'    888   888  888   888   888  888   888  `"Y88b.  
-888  `88b.  888    .o    `888'     888   888  888   888   888  888   888  o.  )88b 
o888o  o888o `Y8bod8P'     .8'      `Y8bod8P' o888o o888o o888o `Y8bod88P" 8""888P' 
-                      .o..P'                                                       
-                      `Y8P'                                                        
*/

fn select_keybinds(data: &mut GameData, user: &mut UserData, em: f32) {
    if is_key_pressed(KeyCode::Escape) {
        user.save();
        data.screen = Screen::Home;
    }
    draw_text("Keybinds", 2.0 * em, 2.0 * em as f32, em * 1.6, DARKGRAY);
    let keys = [
        "up",
        "down",
        "left",
        "right",
        "ability 1",
        "ability 2",
        "ability 3",
        "ability 4",
        "ability 5",
    ];
    let keys2 = vec![
        vec![user.up, user.down, user.left, user.right],
        vec![
            user.ability_key[0],
            user.ability_key[1],
            user.ability_key[2],
            user.ability_key[3],
            user.ability_key[4],
        ],
    ]
    .concat();

    let c = get_last_key_pressed();
    if c != None {
        if is_key_pressed(c.unwrap()) && !is_key_pressed(KeyCode::Escape) {
            data.sounds.push(("click".to_owned(), 0.0));
            match data.keybind_focus as usize {
                0 => user.up = c.unwrap(),
                1 => user.down = c.unwrap(),
                2 => user.left = c.unwrap(),
                3 => user.right = c.unwrap(),
                4 => user.ability_key[0] = c.unwrap(),
                5 => user.ability_key[1] = c.unwrap(),
                6 => user.ability_key[2] = c.unwrap(),
                7 => user.ability_key[3] = c.unwrap(),
                8 => user.ability_key[4] = c.unwrap(),
                _ => {}
            }
            data.keybind_focus = 99.0
        }
    }

    for i in 0..9 {
        if data.keybind_focus == i as f32 {
            draw_text(
                &format!("{}:", keys[i],),
                2.0 * em,
                (i as f32 * 1.3 + 5.0) * em,
                em,
                Color::new(0.5, 0.5, 1.0, 1.0),
            );
            draw_text(
                &format!("{:?}", keys2[i]),
                15.0 * em,
                (i as f32 * 1.3 + 5.0) * em,
                em,
                Color::new(0.5, 0.5, 1.0, 1.0),
            );
        } else {
            draw_text(
                &format!("{}:", keys[i],),
                2.0 * em,
                (i as f32 * 1.3 + 5.0) * em,
                em,
                GRAY,
            );
            draw_text(
                &format!("{:?}", keys2[i]),
                15.0 * em,
                (i as f32 * 1.3 + 5.0) * em,
                em,
                GRAY,
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_y = mouse_position().1 / em;
            if mouse_y > ((i as f32 - 1.0) * 1.3 + 5.0) && mouse_y < (i as f32 * 1.3 + 5.0) {
                data.keybind_focus = i as f32;
                data.sounds.push(("click".to_owned(), 0.0));
            }
        }
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_y = mouse_position().1 / em;
        if mouse_y > (8.0 * 1.3 + 5.0) {
            data.sounds.push(("click".to_owned(), 0.0));

            data.keybind_focus = 99.0;
        }
    }
}


/*
ooooooooooooo                           .                                  
8'   888   `8                         .o8                                  
-    888       .ooooo.  oooo    ooo .o888oo oooo  oooo  oooo d8b  .ooooo.  
-    888      d88' `88b  `88b..8P'    888   `888  `888  `888""8P d88' `88b 
-    888      888ooo888    Y888'      888    888   888   888     888ooo888 
-    888      888    .o  .o8"'88b     888 .  888   888   888     888    .o 
    o888o     `Y8bod8P' o88'   888o   "888"  `V88V"V8P' d888b    `Y8bod8P' 
*/

fn select_texture(data: &mut GameData, user: &mut UserData, em: f32) {
    if is_key_pressed(KeyCode::Escape) {
        data.screen = Screen::Home;
    }
    if fs::read_dir("./res").is_err() {
        draw_text(
            "./res file not found",
            2.0 * em,
            2.0 * em as f32,
            em * 1.6,
            DARKGRAY,
        );
        return;
    }
    let paths = fs::read_dir("./res").unwrap();
    let mut path_str = paths
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|e| e.is_dir())
        .collect::<Vec<_>>();
    path_str.sort();
    path_str.retain(|f| f.to_str().unwrap().replace("./res\\", "") != "audio");

    draw_text(
        "Texture Packs",
        2.0 * em,
        2.0 * em as f32,
        em * 1.6,
        DARKGRAY,
    );
    draw_text(
        format!("current texture pack: {}", user.texture).as_str(),
        2.0 * em,
        19.5 * em as f32,
        em * 0.8,
        DARKGRAY,
    );
    let path = fs::canonicalize(PathBuf::from("./res")).unwrap();

    draw_text(
        &format!("path: {:?}", path).as_str().replace("\\\\", "/"),
        2.0 * em,
        20.5 * em as f32,
        em * 0.8,
        DARKGRAY,
    );
    draw_text("(restart required)", 2.4 * em, 2.5 * em, em / 2.0, RED);

    for i in 0..path_str.len() {
        let x_displacement = (i as f32 / 10.0).floor();
        draw_text(
            &path_str[i].to_str().unwrap().replace("./res\\", ""),
            2.0 * em + (i as f32 / 10.0).floor() + x_displacement * 20.0 * em,
            (i as f32 * em + 3.0 * em) * 1.5 + em - x_displacement * 15.0 * em,
            em * 1.2,
            GRAY,
        );
        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_position().1 > (i as f32 * em + 3.0 * em) * 1.5
                && mouse_position().1 < (i as f32 * em + 3.0 * em) * 1.5 + em
            {
                data.sounds.push(("click".to_owned(), 0.0));

                user.texture = path_str[i].to_str().unwrap().replace("./res\\", "");
                data.screen = Screen::Home;
                user.save();
            }
        }
    }
}

/*
ooooo   ooooo                                       
`888'   `888'                                       
.888     888   .ooooo.  ooo. .oo.  .oo.    .ooooo.  
.888ooooo888  d88' `88b `888P"Y88bP"Y88b  d88' `88b 
.888     888  888   888  888   888   888  888ooo888 
.888     888  888   888  888   888   888  888    .o 
o888o   o888o `Y8bod8P' o888o o888o o888o `Y8bod8P' 
*/

pub fn display_home(em: f32, user: &mut UserData, data: &mut GameData) {
    if data.screen == Screen::AbilitiesScreen {
        select_ability(data, user, em);
        return;
    }
    if data.screen == Screen::Textures {
        select_texture(data, user, em);
        return;
    }
    if data.screen == Screen::Keybinds {
        select_keybinds(data, user, em);
        return;
    }
        if data.screen == Screen::Death {
        draw_death_screen(data, user, em);
        return;
    }

    // let rng = rand::gen_range(0, 10);
    draw_text("Bare King", screen_width()/2.0 - 6.0 * em, 6.0 * em, em * 3.0, Color::from_rgba(150, 200, 100, 200));


    let mouse_y = mouse_position().1;
    let mouse_x = mouse_position().0;

    let start = match data.alive {
            true => "Resume",
            false => "New Game"
    };

    let new_game_tax = match data.alive {
            true => 0.0,
            false => 0.5*em
    };

    draw_text(start, screen_width()/2.0 - 1.6 * em - new_game_tax, 10.0 * em, em*1.2, LIGHTGRAY);
    draw_text("Keybinds", screen_width()/2.0 - 2.1  * em, 12.0 * em, em*1.2, LIGHTGRAY);
    draw_text("Textures", screen_width()/2.0 - 2.1  * em, 14.0 * em, em*1.2, LIGHTGRAY);
    draw_text("Quit", screen_width()/2.0 - 1.1 * em, 16.0 * em, em, RED);



        if  mouse_x > screen_width()/2.0 - 1.6  * em - new_game_tax &&
            mouse_x < screen_width()/2.0 + 1.6  * em + new_game_tax &&
            mouse_y < 10.0*em &&
            mouse_y > 10.0*em - 1.2 * em
                {
                draw_text(start, screen_width()/2.0 - 1.6 * em - new_game_tax, 10.0 * em , em*1.2, WHITE);
                if is_mouse_button_pressed(MouseButton::Left) { 
                    data.sounds.push(("click".to_owned(), 0.0));
                    if data.alive {
                        data.screen = Screen::Game;
                    }else {
                        data.screen = Screen::AbilitiesScreen;
                    }
                }
        }

        if  mouse_x > screen_width()/2.0 - 2.1  * em &&
            mouse_x < screen_width()/2.0 + 2.1  * em &&
            mouse_y < 12.0*em &&
            mouse_y > 12.0*em - 1.2 * em
                {
                draw_text("Keybinds", screen_width()/2.0 - 2.1  * em, 12.0 * em, em*1.2, WHITE);
                if is_mouse_button_pressed(MouseButton::Left) { 
                    data.screen = Screen::Keybinds;
                    data.sounds.push(("click".to_owned(), 0.0));
                }
        }

        if  mouse_x > screen_width()/2.0 - 2.1  * em &&
            mouse_x < screen_width()/2.0 + 2.1  * em &&
            mouse_y < 14.0*em &&
            mouse_y > 14.0*em - 1.2 * em
                {
                draw_text("Textures", screen_width()/2.0 - 2.1  * em, 14.0 * em, em*1.2, WHITE);
                if is_mouse_button_pressed(MouseButton::Left) { 
                    data.screen = Screen::Textures;
                    data.sounds.push(("click".to_owned(), 0.0));
                }
        }
        
        if  mouse_x > screen_width()/2.0 - 1.1  * em &&
            mouse_x < screen_width()/2.0 + 1.1  * em &&
            mouse_y < 16.0*em &&
            mouse_y > 16.0*em - 1.2 * em
                {
                if is_mouse_button_pressed(MouseButton::Left) { 
                    data.sounds.push(("click".to_owned(), 0.0));
                    user.save();
                    if env::consts::OS == "linux" {
                        std::process::exit(0x0100);
                    }
                    std::process::exit(0);
                }
        }



}
