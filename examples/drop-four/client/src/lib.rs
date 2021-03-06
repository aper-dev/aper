#![recursion_limit = "1024"]

use aper_yew::{ClientBuilder, View, ViewContext};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::state::PlayState;
pub use crate::state::{
    Board, DropFourGame, GameTransition, PlayerColor, BOARD_COLS, BOARD_ROWS,
};
use aper::PlayerID;
use board_component::BoardComponent;

mod board_component;
mod state;

#[derive(Clone)]
struct GameView;

impl GameView {
    fn view_waiting(
        &self,
        waiting_player: Option<PlayerID>,
        context: &ViewContext<GameTransition>,
    ) -> Html {
        return html! {
            if Some(context.player) == waiting_player {
                html! {
                    <p>{"Waiting for another player."}</p>
                }
            } else {
                let message = if waiting_player.is_some() {
                    "One player is waiting to play."
                } else {
                    "Nobody is waiting to play."
                };

                html! {
                    <div>
                        <button onclick=context.callback.reform(|_| Some(GameTransition::Join))>{"Join"}</button>
                        <p>{message}</p>
                    </div>
                }
            }
        };
    }

    fn view_playing(
        &self,
        board: &Board,
        next_player: PlayerColor,
        winner: Option<PlayerColor>,
        own_color: Option<PlayerColor>,
        context: &ViewContext<GameTransition>,
    ) -> Html {
        let status_message = if let Some(own_color) = own_color {
            if let Some(winner) = winner {
                if winner == own_color {
                    "Congrats, you are the winner!".to_string()
                } else {
                    format!("{} is the winner. Better luck next time!", winner.name())
                }
            } else if next_player == own_color {
                "It's your turn!".to_string()
            } else {
                format!("It's {}'s turn", next_player.name())
            }
        } else {
            format!("You're observing. {} is next.", next_player.name())
        };

        return html! {
            <div>
                <p>{status_message}</p>
                <BoardComponent
                    board=board
                    player=next_player
                    interactive={Some(next_player)==own_color}
                    callback=context.callback.reform(Some).clone() />
                {
                    if winner.is_some() {
                        html! {
                            <button onclick=context.callback.reform(|_| Some(GameTransition::Reset))>{"New Game"}</button>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        };
    }

    fn view_inner(
        &self,
        state: &DropFourGame,
        context: &ViewContext<GameTransition>,
    ) -> Html {
        match state.state() {
            PlayState::Playing {
                board,
                next_player,
                winner,
                player_map,
                ..
            } => {
                let own_color = player_map.color_of_player(context.player);
                self.view_playing(board, *next_player, *winner, own_color, context)
            }
            PlayState::Waiting { waiting_player, .. } => {
                self.view_waiting(*waiting_player, context)
            }
        }
    }
}

impl View for GameView {
    type Callback = GameTransition;
    type State = DropFourGame;

    fn view(&self, state: &Self::State, context: &ViewContext<Self::Callback>) -> Html {
        return html! {
            <div class="main">
                <h1>{"Drop Four"}</h1>
                { self.view_inner(&state, context) }
            </div>
        };
    }
}

#[wasm_bindgen(start)]
pub fn entry() {
    ClientBuilder::new(GameView).mount_to_body();
}
