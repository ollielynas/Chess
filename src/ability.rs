use crate::audio_g::*;
use crate::game_data::*;
use crate::home::*;
use crate::particles_fnc::*;
use crate::player::*;
use ::rand::prelude::*;
use macroquad::audio::*;
use macroquad::prelude::*;
use strum_macros::EnumIter;

pub fn targeted_ability(data: &mut GameData, point: Coord) {
    let player = Coord {
        x: data.player.x,
        y: data.player.y,
    };
    match data.select_square.ability {
        Abilities::Teleport => {
            data.player.target_x = point.x;
            data.player.target_y = point.y;
            data.player.x = point.x;
            data.player.y = point.y;
        }

        Abilities::Airstrike(_) => {
            data.effects.push((
                Abilities::Airstrike([
                    Coord {
                        x: point.x + thread_rng().gen_range(-30..30) as f32 / 10.0,
                        y: point.y + thread_rng().gen_range(-30..30) as f32 / 10.0,
                    },
                    Coord {
                        x: point.x + thread_rng().gen_range(-30..30) as f32 / 10.0,
                        y: point.y + thread_rng().gen_range(-30..30) as f32 / 10.0,
                    },
                    Coord {
                        x: point.x + thread_rng().gen_range(-30..30) as f32 / 10.0,
                        y: point.y + thread_rng().gen_range(-30..30) as f32 / 10.0,
                    },
                    Coord {
                        x: point.x + thread_rng().gen_range(-30..30) as f32 / 10.0,
                        y: point.y + thread_rng().gen_range(-30..30) as f32 / 10.0,
                    },
                    Coord {
                        x: point.x + thread_rng().gen_range(-30..30) as f32 / 10.0,
                        y: point.y + thread_rng().gen_range(-30..30) as f32 / 10.0,
                    },
                ]),
                2.0,
            ));
        }

        Abilities::Jump => {
            let x_dist = player.x - point.x;
            let y_dist = player.y - point.y;
            if (x_dist.powf(2.0) + y_dist.powf(2.0)).powf(0.5) <= 3.0 {
                data.player.target_x = point.x;
                data.player.target_y = point.y;
            } else {
                data.player.energy += 5.0;
                data.score_text.push(TextReadout {
                    x: point.x - 2.0,
                    y: point.y + 1.0,
                    text: "Too far away".to_string(),
                    lifetime: 40.0,
                })
            }
        }
        Abilities::DeathLaser => {
            println!("death {} {}", player.x, ((player.x + point.x) / 2.0));

            for i in 0..250 {
                let x_cord = player.x + (player.x - point.x) * -1.0 * (i as f32 / 25.0);
                let y_cord = player.y + (player.y - point.y) * -1.0 * (i as f32 / 25.0);
                if !(0.0..=15.0).contains(&x_cord) || !(0.0..=15.0).contains(&y_cord) {
                    break;
                }
                data.bubble_particles.push(Bubble {
                    x: x_cord + 1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                    y: y_cord + 1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                    color: [thread_rng().gen_range(200..250) as f32, 0.0, 0.0, 250.0],
                    r: thread_rng().gen_range(10..20) as f32 / 100.0,
                    decay: 0.0,
                    x_velocity: 0.0,
                    y_velocity: 0.0,
                    lifetime: thread_rng().gen_range(10..200) as f32,
                });

                let yellow = thread_rng().gen_range(100..200) as f32;
                data.bubble_particles.push(Bubble {
                    x: x_cord + 1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                    y: y_cord + 1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                    color: [yellow, yellow, 0.0, 125.0],
                    r: thread_rng().gen_range(20..30) as f32 / 100.0,
                    decay: 0.001,
                    x_velocity: 0.0,
                    y_velocity: 0.0,
                    lifetime: thread_rng().gen_range(10..200) as f32,
                });

                data.enemies.retain(|f| {
                    ((f.x.abs() - x_cord.abs()).abs() + (f.y.abs() - y_cord.abs()).abs()).powf(0.5)
                        > 1.2
                })
            }
        }
        _ => {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Savefile, EnumIter)]
pub enum Abilities {
    Null,
    RBlast,
    Blip,
    Jump,
    DeathLaser,
    Bank,
    Teleport,
    BloodBath,
    Peaceful,
    Martyrdom,
    Airstrike([Coord; 5]),
    WallOfFire([Coord; 5]),
    Redstone,
    ColdStorage,
    ShortTermGains,
    Dope,
}

pub const FIRE_COLORS: [[f32; 4]; 5] = [
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
    pub duration: Option<u32>,
}
/// contains the data for each ability
pub fn metadata(a: Abilities) -> AbilityMetadata {
    match a {
        Abilities::Airstrike(_) => AbilityMetadata {
            name: "Airstrike".to_string(),
            description:
                "Destroys pieces in a large radius anywhere on the board with a 2 move delay"
                    .to_string(),
            cost: 20,
            duration: Some(3),
        },
        Abilities::ShortTermGains => AbilityMetadata {
            name: "Crippling Debt".to_string(),
            description:
                "+ 30 energy, but you have to pay it back 2 energy at a time. If you cannot you die"
                    .to_string(),
            cost: -30,
            duration: Some(15),
        },
        Abilities::Redstone => AbilityMetadata {
            name: "Redstone Powder".to_string(),
            description: "Increases the duration of all active effects by 2".to_string(),
            cost: 5,
            duration: None,
        },
        Abilities::ColdStorage => AbilityMetadata {
            name: "Cold Storage".to_string(),
            description:
                "Activate ability to store 5 energy. Active a second time to get 10 energy back"
                    .to_string(),
            cost: 5,
            duration: Some(9999),
        },
        Abilities::WallOfFire(_) => AbilityMetadata {
            name: "Wall Of Fire".to_string(),
            description: "Spawns a wall that pieces cannot move through (not added yet)"
                .to_string(),
            cost: 10,
            duration: Some(10),
        },
        Abilities::Peaceful => AbilityMetadata {
            name: "Calming Lull".to_string(),
            description: "Decreases enemy spawn rate for 15 rounds".to_string(),
            cost: 10,
            duration: Some(15),
        },
        Abilities::Martyrdom => AbilityMetadata {
            name: "Martyrdom".to_string(),
            description: "Kills Everything. You die on the next turn. Adds 0.5x score multiplayer"
                .to_string(),
            cost: 0,
            duration: Some(1),
        },
        Abilities::BloodBath => AbilityMetadata {
            name: "Blood Bath".to_string(),
            description: "Double the effect of the multi kill multiplier (cannot stack)"
                .to_string(),
            cost: 5,
            duration: Some(3),
        },
        Abilities::Jump => AbilityMetadata {
            name: "Jump".to_string(),
            description: "Allows the player to move to any space in a 3 square radius".to_string(),
            cost: 5,
            duration: None,
        },
        Abilities::Teleport => AbilityMetadata {
            name: "Teleport".to_string(),
            description: "The ultimate open, move anywhere on the board".to_string(),
            cost: 15,
            duration: None,
        },
        Abilities::Bank => AbilityMetadata {
            name: "Investment Banking".to_string(),
            description: "Returns 10 energy in 5 rounds".to_string(),
            cost: 5,
            duration: Some(5),
        },
        Abilities::Blip => AbilityMetadata {
            name: "blip".to_owned(),
            description:
                "spawn 5 new pieces. counts as a move. has a negative effect on active effects."
                    .to_owned(),
            cost: -2,
            duration: None,
        },
        Abilities::RBlast => AbilityMetadata {
            name: "radial blast".to_owned(),
            description: "kills everything in a circle around the player".to_owned(),
            cost: 10,
            duration: None,
        },
        Abilities::DeathLaser => AbilityMetadata {
            name: "death laser".to_owned(),
            description: "can be shot at any angle, kills everything in its path".to_owned(),
            cost: 15,
            duration: None,
        },
        Abilities::Dope => AbilityMetadata {
            name: "Dope".to_owned(),
            description: "Get +1 max energy for every passive effect active (will not stack)"
                .to_owned(),
            cost: 10,
            duration: Some(9999),
        },
        _ => AbilityMetadata {
            name: "no ability selected".to_owned(),
            description: "no ability has been selected".to_owned(),
            cost: 99,
            duration: None,
        },
    }
}

pub fn activate_ability(ability: Abilities, data: &mut GameData, _user: &UserData) {
    if data.player.energy >= metadata(ability).cost as f32 {
        data.player.energy -= metadata(ability).cost as f32
    } else {
        return;
    };

    match ability {
        Abilities::Jump => {
            data.select_square = SelectSquare {
                point: Coord { x: 0.0, y: 0.0 },
                select_mode: true,
                read: false,
                ability: Abilities::Jump,
            };
        }
        Abilities::Redstone => {
            for effect in &mut data.effects {
                effect.1 += 2.0;
            }
        }
        Abilities::Martyrdom => {
            data.effects.push((Abilities::Martyrdom, 1.0));
            for enemy in &data.enemies {
                let l = thread_rng().gen_range(10..30);
                let r = thread_rng().gen_range(10..30);
                data.bubble_particles.push(Bubble {
                    x: enemy.x
                        + 1.5
                        + thread_rng().gen_range(-10..10) as f32 / 100.0,
                    y: enemy.y
                        + 1.5
                        + thread_rng().gen_range(-10..10) as f32 / 100.0,
                    color: [0.0,0.0,0.0,thread_rng().gen_range(0..10) as f32 / 10.0 * 255.0],
                    r: r as f32 / 100.0,
                    decay: (r as f32 / l as f32) / 100.0,
                    x_velocity: thread_rng().gen_range(-15..15) as f32 / 5000.0,
                    y_velocity: thread_rng().gen_range(-15..0) as f32 / 500.0,
                    lifetime: l as f32,
                });
                println!("bubble");
            }
            println!("{:?}" , data.bubble_particles);
            data.enemies = vec![];
        }
        Abilities::Teleport => {
            data.select_square = SelectSquare {
                point: Coord { x: 0.0, y: 0.0 },
                select_mode: true,
                read: false,
                ability: Abilities::Teleport,
            };
        }
        Abilities::Blip => {
            data.sounds.push(("abilities/warp.wav".to_owned(), 0.4));
            for _ in 0..5 {
                data.spawn_enemy(true);
            }
            for effect in &mut data.effects {
                effect.1 -= 2.0;
                if effect.1 < 0.0 {
                    effect.1 = 0.0
                }
            }
            data.player.sub_round += 5;
        }

        Abilities::ColdStorage => {
            let mut retrieve = true;
            for i in 0..data.effects.len() {
                if data.effects[i].0 == Abilities::ColdStorage {
                    retrieve = false;
                    data.player.energy += 10.0;
                    data.effects.remove(i);
                }
            }
            if retrieve {
                data.effects.push((Abilities::ColdStorage, 9999.0));
            }
        }

        Abilities::RBlast => {
            data.sounds.push(("abilities/fire.wav".to_owned(), 1.0));
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
                .retain(|f| !blast_area.contains(&Coord { x: f.x, y: f.y }));
        }
        Abilities::DeathLaser => {
            data.select_square = SelectSquare {
                point: Coord { x: 0.0, y: 0.0 },
                select_mode: true,
                read: false,
                ability: Abilities::DeathLaser,
            }
        }
        Abilities::Airstrike(a) => {
            data.select_square = SelectSquare {
                point: Coord { x: 0.0, y: 0.0 },
                select_mode: true,
                read: false,
                ability: Abilities::Airstrike(a),
            }
        }
        _ => match metadata(ability).duration {
            Some(a) => data.effects.push((ability, a as f32)),
            None => {}
        },
    }
}

/*
ooooooooooooo           o8o                                                           .o88o.  .o88o.                         .
8'   888   `8           `"'                                                           888 `"  888 `"                       .o8
-    888      oooo d8b oooo   .oooooooo  .oooooooo  .ooooo.  oooo d8b       .ooooo.  o888oo  o888oo   .ooooo.   .ooooo.  .o888oo  .oooo.o
-    888      `888""8P `888  888' `88b  888' `88b  d88' `88b `888""8P      d88' `88b  888     888    d88' `88b d88' `"Y8   888   d88(  "8
-    888       888      888  888   888  888   888  888ooo888  888          888ooo888  888     888    888ooo888 888         888   `"Y88b.
-    888       888      888  `88bod8P'  `88bod8P'  888    .o  888          888    .o  888     888    888    .o 888   .o8   888 . o.  )88b
    o888o     d888b    o888o `8oooooo.  `8oooooo.  `Y8bod8P' d888b         `Y8bod8P' o888o   o888o   `Y8bod8P' `Y8bod8P'   "888" 8""888P'
-                            d"     YD  d"     YD
-                            "Y88888P'  "Y88888P'
*/

pub fn trigger_effects(data: &mut GameData) {
    for effect in &mut data.effects {
        match effect.0 {
            Abilities::Bank => {
                if effect.1 <= 0.0 {
                    data.player.energy += 10.0;
                    if data.player.energy > 30.0 {
                        data.player.energy = 30.0;
                    }
                }
            }
            Abilities::Martyrdom => {
                if effect.1 <= 0.0 {
                    data.alive = false;
                }
            }
            Abilities::ShortTermGains => {
                if data.player.energy >= 2.0 {
                    data.player.energy -= 2.0;
                } else {
                    data.alive = false;
                }
            }
            Abilities::Airstrike(b) => {
                if effect.1 <= 0.0 {
                    for i in b {
                        data.enemies.retain(|f| {
                            ((f.x - i.x).powi(2) + (f.y - i.y).powi(2)).powf(0.5) > 3.0
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
