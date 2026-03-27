use macroquad::prelude::*;

#[macroquad::main("Mon RPG en Rust")]
async fn main() {
    // 1. Chargement d'une image (équivalent à ImageIO.read en Java)
    // Place ton image dans un dossier "assets/" à la racine de ton projet
    let texture: Texture2D = load_texture("assets/player.png").await.unwrap();

    let mut x = 100.0;
    let mut y = 100.0;

    loop {
        // --- LOGIQUE (Update) ---
        if is_key_down(KeyCode::Right) { x += 5.0; }
        if is_key_down(KeyCode::Left) { x -= 5.0; }
        if is_key_down(KeyCode::Up) { y -= 5.0; }
        if is_key_down(KeyCode::Down) { y += 5.0; }

        // --- DESSIN (Render) ---
        clear_background(BLACK); // On nettoie l'écran (comme repaint())

        // Dessiner un pixel ou un rectangle de couleur
        draw_rectangle(x, y, 50.0, 50.0, RED);

        // Dessiner une image (Texture)
        draw_texture(&texture, x + 60.0, y, WHITE);

        // Afficher du texte
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, GREEN);

        next_frame().await // On attend la frame suivante (60 FPS par défaut)
    }
}