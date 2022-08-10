use macroquad::prelude::*;
mod player;
use player::*;
mod game_data;
use game_data::*;
mod home;
use home::*;
// use macroquad_particles::{self as particles, AtlasConfig, BlendMode, Emitter, EmitterConfig};
// mod particles_fnc;
// use particles_fnc::*;

const DEFAULT_GAME_STATE: GameData = GameData {
        player: Player {
            x: 8.0,
            y: 8.0,
            target_x: 8.0,
            target_y: 8.0,
            sub_round: 3,
            energy: 0.0,
        },
        round: 0,
        enemies: vec![],
        alive: false
};

fn vect_difference(v1: &Vec<Enemy>, v2: &Vec<Enemy>) -> Vec<Enemy> {
    v1.iter().filter(|&x| !v2.contains(x)).cloned().collect()
}

async fn load_local_texture(id: String) -> Texture2D {
    return load_texture(&format!("./src/res/{}.png", id)).await.unwrap();
}

#[macroquad::main("Chess")]
async fn main() {
    let mut em = (screen_height() / 32.0) * 1.5;
    let mut dsp_square = DrawTextureParams {
        // 32x32
        dest_size: Some(vec2(em, em)),
        ..Default::default()
    };
    




    let mut dsp_piece = DrawTextureParams {
        // 32x42
        dest_size: Some(vec2(em, em + 10.0 * (em / 32.0))),
        ..Default::default()
    };

    let user = UserData {
        left: KeyCode::A,
        right: KeyCode::D,
        up: KeyCode::W,
        down: KeyCode::S,
    };

    let mut game_data = DEFAULT_GAME_STATE;

    let select: Texture2D = load_local_texture("select".to_owned()).await;
    let black_square: Texture2D = load_local_texture("black_square".to_owned()).await;
    let white_square: Texture2D = load_local_texture("white_square".to_owned()).await;
    let pawn_texture: Texture2D = load_local_texture("pawn".to_owned()).await;
    let red_square: Texture2D = load_local_texture("red_square".to_owned()).await;
    let rook_texture: Texture2D = load_local_texture("rook".to_owned()).await;
    let bishop_texture: Texture2D = load_local_texture("bishop".to_owned()).await;
    let mut size = 0.0;

    loop {

        // define variable em
        em = (screen_height() / 32.0) * 1.5;

        // change piece and square sizes of em has changed
        if size != em {
            dsp_square.dest_size = Some(vec2(em, em));
            dsp_piece.dest_size = Some(vec2(em, em + em / 3.2));
        }
        
        clear_background(BLACK);


        let mouse_x = mouse_position().0/em;
        let mouse_y = mouse_position().1/em;

        // when player dies or start new game
        if !game_data.alive {
            display_home(em);
            if is_key_down(KeyCode::Enter) {
                game_data = DEFAULT_GAME_STATE;
                game_data.alive = true;
            }
            next_frame().await;
        }else { // chose to display home or game

        let selected_square_x = (mouse_x -1.5).round();
        let selected_square_y = (mouse_y -1.5).round();

        // draw board
        for i in 0..16 {
            for j in 0..16 {
                // pick between black and white
                let mut tex = black_square;
                if (i + j) % 2 == 0 {
                } else {
                    tex = white_square;
                }
                // draw square
                draw_texture_ex(
                    tex,
                    i as f32 * em + em,
                    j as f32 * em + em,
                    WHITE,
                    dsp_square.clone(),
                );

                if j as f32 == game_data.player.target_y && i as f32 == game_data.player.target_x {
                    draw_texture_ex(
                        select,
                        i as f32 * em + em,
                        j as f32 * em + em,
                        WHITE,
                        dsp_square.clone(),
                    );
                }
            }
        }



        // get user input then make move
        if player_movement(&mut game_data.player, &user) {
            game_data.player.sub_round += 1;
            
            let e_1 = game_data.enemies.clone();
            game_data.enemies.retain(|e| e.x != game_data.player.target_x || e.y != game_data.player.target_y);
            
            if vect_difference(&e_1, &game_data.enemies).len() > 0 {
                game_data.player.energy += match vect_difference(&e_1, &game_data.enemies)[0].piece {
                    Piece::Pawn => 1.0,
                    Piece::Rook => 3.0,
                    Piece::Bishop => 5.0,
                    _ => 5.0
                };

                if game_data.player.energy >= 20.0 {
                    game_data.player.energy = 20.0;
                }
            }
            
            if game_data.player.sub_round >= 3 {
                game_data.player.sub_round = 0;
                game_data.round += 1;
                game_data.enemy_move();
                game_data.spawn_enemy();
            }
            
        }
    
        
        for i in &game_data.enemies {
            for j in &i.moves {
            draw_texture_ex(
                red_square,
                j.x * em + em,
                j.y * em + em,
                WHITE,
                dsp_square.clone(),
            );
        }
        
    }



    for i in &game_data.enemies {
                if i.moves.len() > 1 {
                for j in 0..i.moves.len() -1 {
                    draw_line(
                        i.moves[j].x*em + em*1.5, 
                        i.moves[j].y*em + em*1.5, 
                        i.moves[j +1].x*em + em*1.5,
                        i.moves[j +1].y*em + em*1.5, 
                        em*0.1, 
                        BLACK);
                        if j == i.moves.len()-2 {
                        let mut shift_x1 = em*0.8;
                        let mut shift_y1 = em*0.8;
                        let mut shift_x2 = em*0.8;
                        let mut shift_y2 = em*0.8;

                        // drawing arrows
                        if i.moves[j+1].y > i.moves[j].y {
                            shift_x2 = em*0.2;
                            shift_y1 = em*0.2;
                            shift_y2 = em*0.2;
                        }
                        else if i.moves[j+1].y < i.moves[j].y {
                            shift_x2 = em*0.2;
                        }
                        else if i.moves[j+1].x < i.moves[j].x {
                            shift_y2 = em*0.2;
                        }
                        else if i.moves[j+1].x > i.moves[j].x {
                            shift_y2 = em*0.2;
                            shift_x1 = em*0.2;
                            shift_x2 = em*0.2;
                        }

                        draw_line(
                            i.moves[j+1].x*em + em*1.5, 
                            i.moves[j+1].y*em + em*1.5, 
                            i.moves[j+1].x*em + em +shift_x1,
                            i.moves[j+1].y*em + em +shift_y1, 
                            em*0.1, 
                            BLACK);
                        draw_line(
                            i.moves[j+1].x*em + em*1.5, 
                            i.moves[j+1].y*em + em*1.5, 
                            i.moves[j+1].x*em + em + shift_x2,
                            i.moves[j+1].y*em + em + shift_y2, 
                            em*0.1, 
                            BLACK);
                        draw_circle(
                            i.moves[j+1].x*em + em*1.5, 
                            i.moves[j+1].y*em + em*1.5, em*0.05, BLACK);
                        
                        draw_circle(
                            i.moves[j+1].x*em + em + shift_x1,
                            i.moves[j+1].y*em + em + shift_y1,
                            em*0.05, BLACK);
                        
                        draw_circle(
                            i.moves[j+1].x*em + em + shift_x2,
                            i.moves[j+1].y*em + em + shift_y2,
                            em*0.05, BLACK);
                }
                }
            }
    }

    for i in &game_data.enemies {
        // draw piece
            draw_texture_ex(
            match &i.piece {
                Piece::Pawn => pawn_texture,
                Piece::Rook => rook_texture,
                Piece::Bishop => bishop_texture,
                _ => pawn_texture
            },
            i.x * em + em,
            i.y * em + em * 0.5,
            WHITE,
            dsp_piece.clone(),
        );
    
    }
    game_data.player.update_pos();
    draw_texture_ex(
        game_data.player.texture().await,
        game_data.player.x * em + em,
        game_data.player.y * em + em * 0.5,
        WHITE,
        dsp_piece.clone(),
    );

        draw_text(
            &format!("energy: {}", game_data.player.energy),
            em * 18.0,
            2.0 * em,
            em * 0.8,
            GREEN,
        );
        let bar = "[".to_owned() + &"I".repeat(game_data.player.energy as usize) + &" ".repeat(20-game_data.player.energy as usize) + "]";
        draw_text(
            &format!(
                "{}",
                bar
            ),
            em * 18.0,
            3.0 * em,
            em * 0.8,
            GRAY,
        );



        draw_text(
            "Move:",
            em * 18.0,
            4.0 * em,
            em * 0.8,
            GRAY,
        );
        draw_text("1",em * 20.0,4.0 * em,em * 0.8,
        match game_data.player.sub_round {
            0 => ORANGE,
            _ => GRAY
        });
        draw_text("2",em * 21.0,4.0 * em,em * 0.8,
        match game_data.player.sub_round {
            1 => ORANGE,
            _ => GRAY
        });
        draw_text("3",em * 22.0,4.0 * em,em * 0.8,
        match game_data.player.sub_round {
            2 => RED,
            _ => GRAY
        });
        draw_text("Round: ",em * 18.0,5.0 * em,em * 0.8,
        GRAY
        );
        draw_text(&format!("{}", game_data.round),em * 20.5,5.0 * em,em * 0.8,
        RED
        );

        for i in &game_data.enemies {
            if i.x == selected_square_x && i.y == selected_square_y {
                                for j in 0..i.moves.len() -1 {
                    draw_line(
                        i.moves[j].x*em + em*1.5, 
                        i.moves[j].y*em + em*1.5, 
                        i.moves[j +1].x*em + em*1.5,
                        i.moves[j +1].y*em + em*1.5, 
                        em*0.1, 
                        BLUE);
                        if j == i.moves.len()-2 {
                        let mut shift_x1 = em*0.8;
                        let mut shift_y1 = em*0.8;
                        let mut shift_x2 = em*0.8;
                        let mut shift_y2 = em*0.8;

                        // drawing arrows
                        if i.moves[j+1].y > i.moves[j].y {
                            shift_x2 = em*0.2;
                            shift_y1 = em*0.2;
                            shift_y2 = em*0.2;
                        }
                        else if i.moves[j+1].y < i.moves[j].y {
                            shift_x2 = em*0.2;
                        }
                        else if i.moves[j+1].x < i.moves[j].x {
                            shift_y2 = em*0.2;
                        }
                        else if i.moves[j+1].x > i.moves[j].x {
                            shift_y2 = em*0.2;
                            shift_x1 = em*0.2;
                            shift_x2 = em*0.2;
                        }

                        draw_line(
                            i.moves[j+1].x*em + em*1.5, 
                            i.moves[j+1].y*em + em*1.5, 
                            i.moves[j+1].x*em + em +shift_x1,
                            i.moves[j+1].y*em + em +shift_y1, 
                            em*0.1, 
                            BLUE);
                        draw_line(
                            i.moves[j+1].x*em + em*1.5, 
                            i.moves[j+1].y*em + em*1.5, 
                            i.moves[j+1].x*em + em + shift_x2,
                            i.moves[j+1].y*em + em + shift_y2, 
                            em*0.1, 
                            BLUE);
                        draw_circle(
                            i.moves[j+1].x*em + em*1.5, 
                            i.moves[j+1].y*em + em*1.5, em*0.05, BLUE);
                        
                        draw_circle(
                            i.moves[j+1].x*em + em + shift_x1,
                            i.moves[j+1].y*em + em + shift_y1,
                            em*0.05, BLUE);
                        
                        draw_circle(
                            i.moves[j+1].x*em + em + shift_x2,
                            i.moves[j+1].y*em + em + shift_y2,
                            em*0.05, BLUE);
                }
                }
            }
        }

        size = em;
        next_frame().await
    }}
}
