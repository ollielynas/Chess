use macroquad::{prelude::*};
mod player;
use player::*;
mod game_data;
use game_data::*;
mod home;
use home::*;
mod particles_fnc;
use particles_fnc::*;
use std::path::Path;
extern crate savefile;
use savefile::prelude::*;
use std::time::Instant;
use std::env;
mod key_map;
mod ability;
use ability::*;
use ::rand::prelude::*;


pub const GLOBAL_VERSION:u32 = 1;


#[macro_use]
extern crate savefile_derive;

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
        alive: false,
        bubble_particles: vec![],
        select_ability: SelectAbility {
            slot: 1,
            open: false,
            page: 0,
        },
        pause: false,
        select_square: SelectSquare {
            point: Coord {x:0.0, y:0.0},
            read: false,
            select_mode: false,
            ability: Abilities::Null,
        },
        score: 0.0,
        score_text: vec![],
};



fn default_user_values() -> UserData {UserData {
        left: KeyCode::A,
        right: KeyCode::D,
        up: KeyCode::W,
        down: KeyCode::S,
        abilities: [
            Abilities::Null,
            Abilities::Null,
            Abilities::Null,
            Abilities::Null,
            Abilities::Null,
        ],
        ability_key: [
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::Key5,
            ],
        texture: "fonky-monky".to_owned(),
}}

fn vect_difference(v1: &Vec<Enemy>, v2: &Vec<Enemy>) -> Vec<Enemy> {
    v1.iter().filter(|&x| !v2.contains(x)).cloned().collect()
}

async fn load_local_texture(id: String, user: &UserData) -> Texture2D {
    if Path::new(&format!("./src/res/{}/{}.png",user.texture, id)).exists() {
        return load_texture(&format!("./src/res/{}/{}.png",user.texture, id)).await.unwrap();        
    }else {
        let default_texture = "Programmer Art".to_owned();
        return load_texture(&format!("./src/res/{}/{}.png",default_texture, id)).await.unwrap(); 
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Bare King".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}



#[macroquad::main(window_conf)]
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

    
    
    let mut user: UserData = default_user_values();
    user.load();

    let select: Texture2D = load_local_texture("select".to_owned(), &user).await;
    let black_square: Texture2D = load_local_texture("black_square".to_owned(), &user).await;
    let white_square: Texture2D = load_local_texture("white_square".to_owned(), &user).await;
    let pawn_texture: Texture2D = load_local_texture("pawn".to_owned(), &user).await;
    let red_square: Texture2D = load_local_texture("red_square".to_owned(), &user).await;
    let rook_texture: Texture2D = load_local_texture("rook".to_owned(), &user).await;
    let bishop_texture: Texture2D = load_local_texture("bishop".to_owned(), &user).await;
    let player_texture: Texture2D = load_local_texture("player_king".to_owned(), &user).await;
    let knight_texture: Texture2D = load_local_texture("knight".to_owned(), &user).await;
    let queen_texture: Texture2D = load_local_texture("queen".to_owned(), &user).await;
    let mut size = 0.0;




    let mut game_data: GameData = match  savefile::load_file("game_data.bin", GLOBAL_VERSION) {
        Ok(e) => {e},
        Err(_) => {
            save_file("game_data.bin", GLOBAL_VERSION, &DEFAULT_GAME_STATE).unwrap();
            println!("failed to read file");
            DEFAULT_GAME_STATE
        }
    };



    let mut save_timer = Instant::now();
    loop {

        if is_quit_requested() {
            save_file("game_data.bin", GLOBAL_VERSION, &game_data).unwrap();
            user.save();
        }


        if save_timer.elapsed().as_secs() > 5 {
            save_timer = Instant::now();
            save_file("game_data.bin", GLOBAL_VERSION, &game_data).unwrap();
            user.save();
        }

        em = (screen_height() / 32.0) * 1.5;

        // change piece and square sizes of em has changed
        
        clear_background(BLACK);
        
        if is_key_pressed(KeyCode::Escape) {
            game_data.pause = !game_data.pause;
        }

        let selecting = game_data.select_square.select_mode;
        
        
        let mouse_x = mouse_position().0/em;
        let mouse_y = mouse_position().1/em;

        // when player dies or start new game
        if !game_data.alive {
            display_home(em, &mut user, &mut game_data);
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
                    
                //     if j as f32 == game_data.player.target_y && i as f32 == game_data.player.target_x {
                //         draw_texture_ex(
                //             select,
                //             i as f32 * em + em,
                //             j as f32 * em + em,
                //             WHITE,
                //             dsp_square.clone(),
                //         );
                // }
            }
        }


        if size != em {
            dsp_square.dest_size = Some(vec2(em, em));
            dsp_piece.dest_size = Some(vec2(em, em + em / 3.2));
        }

        let mut killed_pieces: Vec<Enemy> = vec![];

        if !game_data.pause && !selecting {
        for i in 0..5 {
            if is_key_pressed(user.ability_key[i]) {
                let starting_pieces = game_data.enemies.clone();
                activate_ability(user.abilities[i], &mut game_data);
                killed_pieces = [killed_pieces, vect_difference(&starting_pieces, &game_data.enemies)].concat();
            }
        }
        }


        // get user input then make move
        if !game_data.pause && !selecting && (player_movement(&mut game_data.player, &user) || game_data.player.sub_round > 3)  {
            game_data.player.sub_round += 1;
            
            let e_1 = game_data.enemies.clone();
            game_data.enemies.retain(|e| e.x != game_data.player.target_x || e.y != game_data.player.target_y);
            
            if vect_difference(&e_1, &game_data.enemies).len() > 0 {
                killed_pieces.push(vect_difference(&e_1, &game_data.enemies)[0].clone());
                game_data.player.energy += match vect_difference(&e_1, &game_data.enemies)[0].piece {
                    Piece::Pawn => 1.0,
                    Piece::Rook => 2.0,
                    Piece::Knight => 2.0,
                    Piece::Bishop => 3.0,
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
                Piece::Knight => knight_texture,
                Piece::Queen => queen_texture,
                _ => pawn_texture
            },
            i.x * em + em,
            i.y * em + em * 0.5,
            WHITE,
            dsp_piece.clone(),
        );
    
    }
    game_data.player.update_pos();

    for i in &mut game_data.bubble_particles {
        draw_circle(i.x*em, i.y*em, i.r*em, Color {r: i.color[0]/255.0, g: i.color[1]/255.0, b: i.color[2]/255.0, a: i.color[3]/255.0});
        i.x +=i.x_velocity;
        i.y += i.y_velocity;
        i.r -= i.decay;
        i.lifetime -= 1.0;
    }
    game_data.bubble_particles.retain(|f| f.lifetime > 0.0);




    draw_texture_ex(
        player_texture,
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
        draw_text(
            &format!("Score: {}", (game_data.score*100.0).round()/100.0),
            em * 18.0,
            6.0 * em,
            em * 0.8,
            GOLD,
        );

        if game_data.player.energy > 20.0 {game_data.player.energy = 20.0};
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

        for i in 0..5 {
            let mut color = GRAY;
            if metadata(user.abilities[i]).cost as f32 <= game_data.player.energy {color = GREEN}
            draw_text(&format!("{:?}", user.ability_key[i]),
            18.0*em,
            (i+7) as f32*em,
            em*0.8,
            color,
            );
            draw_text(&format!("| {:?}", metadata(user.abilities[i]).cost),
            19.5*em,
            (i+7) as f32*em,
            em*0.8,
            color,
            );
            draw_text( &("| ".to_owned()+&metadata(user.abilities[i]).name),
            21.0*em,
            (i+7) as f32*em,
            em*0.8,
            color,
            );
        }

        if selected_square_x > 16.0 {
        for i in 0..5 {
                if mouse_y.round() as usize  == i + 7 {
                    draw_text( &metadata(user.abilities[i]).description,
                    em,
                    18.0*em,
                    em*0.8,
                    GRAY,
                    );
                    draw_text( "|",
                    17.5*em,
                    (i + 7) as f32*em,
                    em*0.8,
                    GRAY,
                    );
                }
            }
        }


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
        // draw pause menu
        if game_data.pause {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color {r:0.0,g:0.0,b:0.0,a:0.9}
            );
            draw_text (
                "paused",
                screen_width()/2.0 - 5.0*em,
                screen_height()/3.0,
                em*3.0,
                GRAY
            );
            draw_text (
                "Quit",
                15.0*em,
                screen_height()/2.0,
                em*2.0,
                RED
            );
            draw_text (
                "Home",
                15.0*em,
                screen_height()/2.0 + em*2.0,
                em*2.0,
                GREEN
            );
            if is_mouse_button_pressed(MouseButton::Left) {
                println!("{} {}", mouse_x, mouse_y);
                if mouse_x > 15.0 && mouse_x < 18.3 && mouse_y > 9.7 && mouse_y < 10.6{
                    save_file("game_data.bin", GLOBAL_VERSION, &game_data).unwrap();
                    user.save();
                    if env::consts::OS == "linux" {
                        std::process::exit(0x0100);
                    }
                    std::process::exit(0);
                }
                if mouse_x > 15.0 && mouse_x < 18.3 && mouse_y > 11.7 && mouse_y < 13.6{
                    save_file("game_data.bin", GLOBAL_VERSION, &game_data).unwrap();
                    game_data.alive = false;
                }
            }
        }
        if game_data.select_square.select_mode {
            draw_texture_ex(
                select,
                selected_square_x * em + em,
                selected_square_y * em + em,
                WHITE,
                dsp_square.clone(),
            );
        if is_mouse_button_pressed(MouseButton::Left) 
        && selected_square_x >= 0.0 
        && selected_square_x <= 15.0
        && selected_square_y >= 0.0
        && selected_square_y <= 15.0
        {
            let starting_pieces = game_data.enemies.clone();
            targeted_ability(
                &mut game_data, 
                Coord {x: selected_square_x as f32, y: selected_square_y as f32} , em
            );
            game_data.select_square = SelectSquare {..Default::default()};
            killed_pieces = [killed_pieces, vect_difference(&starting_pieces, &game_data.enemies)].concat();
        }
        }


        let mut score_multiplier = 1.0+(killed_pieces.len() as f32 *0.1);
        if killed_pieces.len() == 1 {
            score_multiplier = 1.0;
        }
        let startscore = game_data.score;
        for p in &killed_pieces {
            let piece_value = match p.piece {
                Piece::Pawn => 2.0,
                Piece::Knight => 5.0,
                Piece::Rook => 5.0,
                Piece::Bishop => 7.0,
                Piece:: Queen => 10.0,
                Piece::King => 12.0,
                };
            game_data.score += piece_value * score_multiplier;

            let mut text = format!("{}x{}",score_multiplier, piece_value);
            if killed_pieces.len() == 1 {
                text = format!("{}", piece_value)
            }
            
            game_data.score_text.push(
                TextReadout {
                    x: p.x + thread_rng().gen_range(1..10)as f32 /10.0,
                    y: p.y + thread_rng().gen_range(-5..5)as f32 /10.0,
                    text: text,
                    lifetime: 30.0 + score_multiplier * 30.0 + thread_rng().gen_range(1..30)as f32
                }
            );
    }
    if game_data.score - startscore >= 1.0 {
        game_data.score_text.push(
        TextReadout {
            x: 22.0 + thread_rng().gen_range(-10..10)as f32 /100.0,
            y: 5.0 + thread_rng().gen_range(-10..10)as f32 /100.0,
            text: format!("+ {}", ((game_data.score - startscore)*100.0).round()/100.0),
            lifetime: 30.0 + 10.0*(game_data.score - startscore)
        }
    );
    }

    for t in &mut game_data.score_text {
        draw_text(
            t.text.as_str(), 
            t.x*em +em,
            t.y*em +em,
            em*0.8, 
            RED
        );
        draw_text(
            t.text.as_str(), 
            (t.x+0.03)*em +em,
            (t.y+0.03)*em +em,
            em*0.8, 
            GOLD
        );
        t.lifetime -= 1.0;
    }
    game_data.score_text.retain(|f| f.lifetime >= 0.0);


        // select a square

        size = em.clone();
        next_frame().await
    }}
}
