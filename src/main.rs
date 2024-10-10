use monopoly::{Game, Player};
use tracing::{instrument, level_filters::LevelFilter, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

fn setup_log() -> (WorkerGuard, WorkerGuard) {
    // Log to file
    let file_appender = tracing_appender::rolling::daily("build/logs", "monopoly.log");
    let (file_writer, file_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::Layer::default()
        .with_writer(file_writer)
        .with_filter(LevelFilter::from_level(Level::INFO));

    // Log to stdout
    let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_layer = tracing_subscriber::fmt::Layer::default().with_writer(stdout_writer);

    Registry::default()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    (file_guard, stdout_guard)
}

#[instrument]
fn main() {
    let (_g1, _g2) = setup_log();

    let mut game = Game::new(vec![Player::new("P1"), Player::new("P2")]);
    game.start_game();
}
