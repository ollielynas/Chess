use crate::GLOBAL_VERSION;
use crate::game_data::*;
use crate::particles_fnc::*;
use ::rand::prelude::*;
use macroquad::prelude::*;
use crate::key_map::*;
use savefile::prelude::*;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
use std::env;


fn key_as_string(key: KeyCode) -> String {
    return format!("{:?}", key);
}

fn string_to_key(key: &String) -> KeyCode {
    for i in KEYS_TEXT {
        if &key_as_string(i) == key {
            return i as KeyCode;
        }
    }
    return KeyCode::Unknown
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
}

#[derive(Savefile)]
struct OtherUserData {
    abilities: [Abilities; 5],
    texture: String,
}

impl UserData {
    fn keybinds_from_vec(&mut self, v: Vec<[String; 2]>) {
        for i in v {
        match i[0].as_str() {
            "left" => {self.left = string_to_key(&i[1]);},
            "right" => {self.right = string_to_key(&i[1]);},
            "up" => {self.up = string_to_key(&i[1]);},
            "down" => {self.down = string_to_key(&i[1])},
            "ability_key" => {
                    self.ability_key = [
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[0].to_owned().replace("[", "")),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[1].to_owned()),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[2].to_owned()),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[3].to_owned()),
                        string_to_key(&i[1].split(", ").collect::<Vec<&str>>()[4].to_owned().replace("]", "")),
                ]; 
            }
            _ => {}
        }
    }
    }

    fn keybinds_to_vec(&self) -> Vec<[String;2]> {
        let ab_keys = format!("{:?}", self.ability_key);
        println!("{}", ab_keys);
        return vec![
            ["left".to_owned() , key_as_string(self.left)],
            ["right".to_owned() , key_as_string(self.right)],
            ["up".to_owned() , key_as_string(self.up)],
            ["down".to_owned() , key_as_string(self.down)],
            ["ability_key".to_owned(), ab_keys]
        ]
    }

    pub fn save(&self) {
        let keys = self.keybinds_to_vec();
        let other = OtherUserData {
            texture: self.texture.clone(),
            abilities: self.abilities.clone()
        };
        save_file("keybinds.bin", GLOBAL_VERSION, &keys).unwrap();
        save_file("user_data.bin", GLOBAL_VERSION, &other).unwrap();
    }

    pub fn load(&mut self) {
        self.keybinds_from_vec(
            match savefile::load_file("keybinds.bin", GLOBAL_VERSION) {
                Ok(e) => e,
                Err(_) => {
                    self.save();
                    vec![]
                }
        });

        let other:OtherUserData;
        match savefile::load_file("user_data.bin", GLOBAL_VERSION) {
                Ok(e) => {
                    other = e;
                    self.abilities = other.abilities;
                    
                    self.texture = other.texture;
                },
                Err(e) => {
                    println!("Error: {}", e);
                    self.save();
                }
            }
        
    }


}



#[derive(Debug, Copy, Clone, PartialEq, Savefile, EnumIter)]
pub enum Abilities {
    Null,
    RBlast,
    Blip,
}

const FIRE_COLORS: [[f32; 4]; 5] = [
    [79.0, 79.0, 79.0, 255.0],
    [11.0, 11.0, 11.0, 255.0],
    [191.0, 58.0, 58.0, 255.0],
    [225.0, 144.0, 61.0, 255.0],
    [216.0, 194.0, 63.0, 255.0],
];

#[derive(Debug, Clone, PartialEq, Savefile)]
pub struct AbilityMetadata {
    pub name: String,
    pub description: String,
    pub cost: i32,
}

pub fn metadata(a: Abilities) -> AbilityMetadata {
    return match a {
        Abilities::Blip => AbilityMetadata {
            name: "blip".to_owned(),
            description: "gain 5 energy. spawn 5 new pieces. counts as a move".to_owned(),
            cost: -5,
        },
        Abilities::RBlast => AbilityMetadata {
            name: "radial blast".to_owned(),
            description: "kills everything in a circle around the player".to_owned(),
            cost: 10,
        },
        _ => AbilityMetadata {
            name: "no ability selected".to_owned(),
            description: "no ability has been selected".to_owned(),
            cost: 99,
        },
    };
}

pub fn activate_ability(ability: Abilities, data: &mut GameData) {
    if data.player.energy >= metadata(ability).cost as f32 {
        data.player.energy -= metadata(ability).cost as f32
    } else {
        return;
    };

    match ability {
        Abilities::Blip => {
            for _ in 0..5 {
                data.spawn_enemy();
                if data.player.energy > 20.0 {
                    data.player.energy = 20.0;
                }
            }
            data.round += 3;
            data.player.sub_round += 3;
        }
        Abilities::RBlast => {
            let mut blast_area: Vec<Coord> = vec![];
            for i in -2..=2 {
                for j in -2..=2 {
                    if !([-2, 2].contains(&i) && [-2, 2].contains(&j)) && (j, i) != (0, 0) {
                        blast_area.push(Coord {
                            x: data.player.x + i as f32,
                            y: data.player.y + j as f32,
                        });
                        for _ in 0..5 {
                            let l = thread_rng().gen_range(10..30);
                            let r = thread_rng().gen_range(10..30);
                            data.bubble_particles.push(Bubble {
                                x: data.player.x
                                    + i as f32
                                    + 1.5
                                    + thread_rng().gen_range(-10..10) as f32 / 100.0,
                                y: data.player.y
                                    + j as f32
                                    + 1.5
                                    + thread_rng().gen_range(-10..10) as f32 / 100.0,
                                color: FIRE_COLORS[thread_rng().gen_range(0..5)],
                                r: r as f32 / 100.0,
                                decay: (r as f32 / l as f32) / 100.0,
                                x_velocity: thread_rng().gen_range(-15..15) as f32 / 5000.0,
                                y_velocity: thread_rng().gen_range(-15..0) as f32 / 500.0,
                                lifetime: l as f32,
                            });
                        }
                    }
                }
            }

            data.enemies
                .retain(|f| !blast_area.contains(&Coord { x: f.x, y: f.y }))
        }
        _ => (),
    }
}

fn select_ability(data: &mut GameData, user: &mut UserData, em:f32) {
    if is_key_pressed(KeyCode::Escape) {
        data.select_ability.open = false;
    }

    let mut o: Vec<Abilities> = Abilities::iter().collect();
    if data.select_ability.page*5 > o.len() || data.select_ability.page < 0 {
        data.select_ability.page = (o.len() as f32/8.0).floor() as usize;
    }

    for _ in 0..data.select_ability.page*5 {
        o.remove(0);
    }

    draw_text("Abilities",2.0*em, 2.0*em as f32,  em*1.6, DARKGRAY);
    let mouse_y = mouse_position().1/em;
    let mouse_x = mouse_position().0/em;

    for i in 1..o.len() {
        draw_text(&format!("{}", metadata(o[i]).name),2.0*em, (((i-1)*2) as f32*em + 3.0*em)*1.5,  em*1.2, GRAY);
        draw_text(&format!("Cost: {}", metadata(o[i]).cost),20.0*em, (((i-1)*2) as f32*em + 3.0*em)*1.5,  em, GRAY);
        draw_text(&format!("{}", metadata(o[i]).description),2.0*em, (((i-1)*2) as f32*em + 3.8*em)*1.5,  em, GRAY);
        if mouse_y > i as f32 *2.7 + 1.0 &&  mouse_y < (i+1)as f32*2.7  + 1.0 {
        draw_text(&format!("{}", metadata(o[i]).name),2.0*em, (((i-1)*2) as f32*em + 3.0*em)*1.5,  em*1.2, LIGHTGRAY);
        draw_text(&format!("Cost: {}", metadata(o[i]).cost),20.0*em, (((i-1)*2) as f32*em + 3.0*em)*1.5,  em, LIGHTGRAY);
        draw_text(&format!("{}", metadata(o[i]).description),2.0*em, (((i-1)*2) as f32*em + 3.8*em)*1.5,  em, LIGHTGRAY);
        if is_mouse_button_pressed(MouseButton::Left) {
            user.abilities[data.select_ability.slot] = o[i];
            data.select_ability.open = false;
            user.save();
        }
        }
    }
}

pub fn display_home(em: f32, user: &mut UserData, data: &mut GameData) {

    if data.select_ability.open {
        select_ability(data, user, em);
        return
    }

    draw_text(
        "Press enter to start...",
        screen_width() - 12.0 * (5.0 + em * 0.7),
        21.0 * em,
        5.0 + em * 0.7,
        DARKGREEN,
    );

    draw_text(
        "Bare King",
        1.0 * em,
        3.0 * em,
    em * 3.0,
        DARKGREEN,
    );
    
    let mouse_y = mouse_position().1/em;
    let mouse_x = mouse_position().0/em;

    draw_text(
        "keybinds",
        em*2.0,
        15.0*em,
        em,
        LIGHTGRAY
    );
    draw_text(
        "textures",
        em*2.0,
        16.5*em,
        em,
        LIGHTGRAY
    );
    draw_text(
        "Quit",
        em*2.0,
        18.0*em,
        em,
        RED
    );

    if is_mouse_button_pressed(MouseButton::Left) {
        if mouse_y > 17.0 && mouse_y < 18.0 && mouse_x > 2.0 && mouse_x < 3.2 {
            if env::consts::OS == "linux" {
                std::process::exit(0x0100);
            }
            std::process::exit(0);        
        }
    }

    for i in 0..5 {
        draw_text(
            &(format!("{}.) ", i+1) + metadata(user.abilities[i]).name.as_str()),
            1.0 * em,
            (6.5 + i as f32) * em,
            4.0 + em,
            GRAY,
        );
        draw_text(
            "Pick New",
            screen_width() / 2.0,
            (6.5 + i as f32) * em,
            em,
            ORANGE,
        );


        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_y > 5.5 + i as f32 
            && mouse_y < 6.5 + i as f32 
            && mouse_position().0 > screen_width() / 2.0
            && mouse_position().0 < (screen_width() / 2.0) + 3.2*em {
                data.select_ability.open = true;
                data.select_ability.slot = i;
            }
        }
    }
}
