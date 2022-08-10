use macroquad::prelude::*;
use crate::game_data::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Abilities {
    Null,
    Blip
}

#[derive(Debug, Clone, PartialEq)]
pub struct AbilityMetadata {
    name: String,
    description: String,
}

pub fn metaData(a:Abilities) -> AbilityMetadata {
    return match a {
        Abilities::Blip => AbilityMetadata {
            name: "blip".to_owned(),
            description: "gain 10 energy. spawn 5 new pieces. counts as a move".to_owned()
        },
        _ => AbilityMetadata {
            name: "no ability selected".to_owned(),
            description: "no ability has been selected".to_owned(),
        }
    }
}

pub fn activate_ability(ability: Abilities, data: &mut GameData) {
    match ability {

        Abilities::Blip => {
            for _ in 0..5 {
                data.spawn_enemy();
                data.player.energy += 2.0;
            }
        }
        _ => ()
    }
}

pub struct UserData {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub ability_key: [KeyCode; 5],
    pub abilities: [Abilities; 5],
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
    
    

    draw_text("abilities",1.0*em,5.0*em,5.0 + em,GRAY);
    draw_text(&format!("1.) {:?}", metaData(user.abilities[0]).name),1.0*em,6.0*em,4.0 + em,GRAY);
    draw_text(&format!("2.) {:?}",  metaData(user.abilities[1]).name),1.0*em,7.0*em,4.0 + em,GRAY);
    draw_text(&format!("3.) {:?}",  metaData(user.abilities[2]).name),1.0*em,8.0*em,4.0 + em,GRAY);
    draw_text(&format!("4.) {:?}",  metaData(user.abilities[3]).name),1.0*em,9.0*em,4.0 + em,GRAY);
    draw_text(&format!("5.) {:?}",  metaData(user.abilities[4]).name),1.0*em,10.0*em,4.0 + em,GRAY);

    draw_text("Pick New",screen_width()/2.0,6.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,7.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,8.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,9.0*em,4.0 + em,ORANGE);
    draw_text("Pick New",screen_width()/2.0,10.0*em,4.0 + em, ORANGE);


}