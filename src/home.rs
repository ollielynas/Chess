use macroquad::prelude::*;
use crate::game_data::*;
use crate::particles_fnc::*;
use ::rand::prelude::*;

pub struct UserData {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub ability_key: [KeyCode; 5],
    pub abilities: [Abilities; 5],
    pub texture: String
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Abilities {
    RBlast,
    Null,
    Blip
}

const FIRE_COLORS: [[f32; 4]; 5] = [
    [79.0,79.0,79.0,255.0],
    [11.0,11.0,11.0, 255.0],
    [191.0,58.0,58.0,255.0],
    [225.0,144.0,61.0,255.0],
    [216.0,194.0,63.0,255.0]
];

#[derive(Debug, Clone, PartialEq)]
pub struct AbilityMetadata {
    pub name: String,
    pub description: String,
    pub cost: i32,
}

pub fn meta_data(a:Abilities) -> AbilityMetadata {
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
        }
    }
}

pub fn activate_ability(ability: Abilities, data: &mut GameData) {

    if data.player.energy >= meta_data(ability).cost as f32 {
        data.player.energy -= meta_data(ability).cost as f32
    }else {return};

    match ability {
        Abilities::Blip => {
            for _ in 0..5 {
                data.spawn_enemy();
                data.player.energy += 2.0;
                if data.player.energy > 20.0 {
                    data.player.energy = 20.0;
                }
            }
            data.round += 3;
            data.player.sub_round += 3;
        },
        Abilities::RBlast => {
            let mut blast_area: Vec<Coord> = vec![];
            for i in -2..=2 {
                for j in -2..=2 {
                    if !([-2,2].contains(&i) && [-2,2].contains(&j)) && (j,i) != (0,0) {
                        blast_area.push(Coord {x: data.player.x + i as f32, y: data.player.y + j as f32});
                        for _ in 0..5 {
                            let l = thread_rng().gen_range(10..30);
                            let r = thread_rng().gen_range(10..30);
                            data.bubble_particles.push(
                                Bubble { x: data.player.x + i as f32 + 1.5 + thread_rng().gen_range(-10..10) as f32/100.0,
                                    y: data.player.y + j as f32 +1.5 + thread_rng().gen_range(-10..10) as f32/100.0,
                                    color: FIRE_COLORS[thread_rng().gen_range(0..5)],
                                    r: r as f32/100.0,
                                    decay: (r as f32/l as f32 )/ 100.0,
                                    x_velocity: thread_rng().gen_range(-15..15) as f32/5000.0,
                                    y_velocity: thread_rng().gen_range(-15..0) as f32/500.0,
                                    lifetime: l as f32 });
                        }
                    }
                }
            }
            
            data.enemies.retain(|f| !blast_area.contains(&Coord {x:f.x,y:f.y}))
        }
        _ => ()
    }
}


pub fn display_home(em: f32, user: &mut UserData) {

    draw_text(
        "Press enter to start...",
            screen_width() - 12.0*(5.0 + em*0.7),
            21.0 * em,
            5.0 + em*0.7,
            DARKGREEN,
        );
        
    draw_text(
        "One True King",
            1.0*em,
            3.0*em,
            8.0 + em*1.5,
            DARKGREEN,
        );
    
    


    for i in 0..5 {
    draw_text(&(format!("{}.) ",i)+ meta_data(user.abilities[i]).name.as_str()), 1.0*em,(6.5 + i as f32)*em,4.0 + em,GRAY);
    draw_text("Pick New",screen_width()/2.0,(6.5+i as f32)*em,4.0 + em,ORANGE);
    }



}