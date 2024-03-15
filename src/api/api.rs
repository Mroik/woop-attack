use tokio::sync::mpsc::UnboundedSender;

use super::message::GameMessage;

struct Api {
    game_tx: UnboundedSender<GameMessage>,
}
