use crate::GLOBAL_VERSION;
use crate::game_data::*;
use crate::particles_fnc::*;
use ::rand::prelude::*;
use macroquad::prelude::*;
use crate::key_map::*;
use savefile::prelude::*;


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
        match savefile::load_file("keybinds.bin", GLOBAL_VERSION) {
                Ok(e) => {
                    other = e;
                    self.abilities = other.abilities;
                    self.texture = other.texture;
                },
                Err(_) => {
                    self.save();
                }
            }
        
    }


}



#[derive(Debug, Copy, Clone, PartialEq, Savefile)]
pub enum Abilities {
    RBlast,
    Null,
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
            description: "gain 10 energy. spawn 5 new pieces. counts as a move".to_owned(),
            cost: 0,
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
                data.player.energy += 1.0;
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
    draw_rectangle(31.0*em, 31.0*em, em, em, Color {r:0.8, g:0.8, b:0.8, a:1.0})
}

pub fn display_home(em: f32, user: &mut UserData, data: &mut GameData) {

    if data.select_ability.open {
        select_ability(data, user, em);
    }

    draw_text(
        "Press enter to start...",
        screen_width() - 12.0 * (5.0 + em * 0.7),
        21.0 * em,
        5.0 + em * 0.7,
        DARKGREEN,
    );

    draw_text(
        "One True King",
        1.0 * em,
        3.0 * em,
        8.0 + em * 1.5,
        DARKGREEN,
    );

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
            4.0 + em,
            ORANGE,
        );

        let mouse_y = mouse_position().1/em;

        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_y > 5.5 + i as f32 
            && mouse_y < 6.5 + i as f32 
            && mouse_position().0 > screen_width() / 2.0
            && mouse_position().0 < (screen_width() / 2.0) + 3.2*em {
                data.select_ability.open = true;
            }
        }
    }
}
