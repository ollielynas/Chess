use std::path::Path;

use macroquad::audio::*;
use crate::game_data::GameData;


pub async fn load_audio(path: String) -> Sound {
    
    if Path::new(&format!("./res/audio/{}",path)).exists() {
        load_sound(&format!("./res/audio/{}", path)).await.unwrap()
    }else{
        println!("file not found");
        load_sound(&format!("./res/audio/{}", path)).await.unwrap()
    }
}

pub async fn load_and_play(path: String, volume: f32) -> bool {
    let a = load_audio(path).await;
    play_sound(
        a,
        PlaySoundParams {
            looped:false,
            volume
        }
    );
    true
}

pub async fn play_sound_stack(data: &mut GameData) {
    for i in &data.sounds {
        let sound = load_audio(i.0.clone()).await;
        play_sound(sound,
            PlaySoundParams { looped: false, volume: i.1 }
        )
    }
    data.sounds.clear();
}