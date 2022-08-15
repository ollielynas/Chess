use crate::player::*;
use macroquad::prelude::*;
use ::rand::prelude::*;
use strum_macros::EnumIter;
use crate::particles_fnc::*;
use crate::home::*;
use crate::game_data::*;

pub fn targeted_ability(data: &mut GameData, point: Coord, em:f32) {
    let player = Coord {x: data.player.x, y: data.player.y};
    match data.select_square.ability {
        Abilities::DeathLaser => {
            println!("death {} {}", player.x, ((player.x+point.x)/2.0));

            for i in 0..250 {
                let x_cord = player.x + (player.x - point.x) * -1.0*(i as f32/25.0);
                let y_cord = player.y + (player.y - point.y) * -1.0*(i as f32/25.0); 
                if x_cord > 15.0 || x_cord < 0.0 || y_cord > 15.0 || y_cord < 0.0 {break}
                data.bubble_particles.push(
                    Bubble { 
                        x: x_cord +1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                        y: y_cord +1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                        color: [thread_rng().gen_range(200..250) as f32
                        ,0.0,0.0,250.0], r: thread_rng().gen_range(10..20) as f32 / 100.0, decay: 0.0,
                        x_velocity: 0.0, y_velocity: 0.0, lifetime: thread_rng().gen_range(10..200) as f32 }
                );

                let yellow = thread_rng().gen_range(100..200) as f32;
                data.bubble_particles.push(
                    Bubble { 
                        x: x_cord + 1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                        y: y_cord + 1.5 + thread_rng().gen_range(-15..15) as f32 / 50.0,
                        color: [yellow ,yellow, 0.0, 125.0], r: thread_rng().gen_range(20..30) as f32 / 100.0, decay: 0.001,
                        x_velocity: 0.0, y_velocity: 0.0, lifetime: thread_rng().gen_range(10..200) as f32 }
                );

                data.enemies.retain(|f| ((f.x.abs() - x_cord.abs()).abs() + (f.y.abs() - y_cord.abs()).abs()).powf(0.5) > 1.5 )
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
    DeathLaser, 
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
        Abilities::DeathLaser => AbilityMetadata {
            name: "death laser".to_owned(),
            description: "can be shot at any angle, kills everything in its path".to_owned(),
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
        },
        Abilities::DeathLaser => {
            data.select_square = SelectSquare { 
                point: Coord {x:0.0, y:0.0},
                select_mode: true,
                read: false,
                ability: Abilities::DeathLaser
            }
        }
        _ => (),
    }
}
