mod entities;
use entities::player::Player;
pub mod components;

use macroquad::prelude::*;

#[macroquad::main("Mon RPG en Rust")]
async fn main() {

    let texture: Texture2D = load_texture("assets/player.png").await.unwrap();

    let x = 100.0;
    let y = 100.0;

    let mut player = Player::new("Héros", x, y);
    print!("{}", player.name);

    loop {
        // --- LOGIQUE (Update) ---
        if is_key_down(KeyCode::Right) { player.pos.x += 5.0; }
        if is_key_down(KeyCode::Left) { player.pos.x -= 5.0; }
        if is_key_down(KeyCode::Up) { player.pos.y -= 5.0; }
        if is_key_down(KeyCode::Down) { player.pos.y += 5.0; }

        // --- DESSIN (Render) ---
        clear_background(BLACK); // On nettoie l'écran (comme repaint())

        // Dessiner une image (Texture)
        draw_texture(&texture, player.pos.x, player.pos.y, WHITE);

        // Afficher du texte
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, GREEN);

        next_frame().await // On attend la frame suivante (60 FPS par défaut).
    }
}