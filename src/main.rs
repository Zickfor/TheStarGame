use star_game::Game;

#[macroquad::main("The Star Game")]
async fn main() {
    let mut game = Game::default().await;
    game.startup().await;
    game.game_loop().await;
}