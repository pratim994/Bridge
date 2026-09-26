use leptos::prelude::*;

use crate::game::{GamePhase, GameSession, Seat, commands::GameCommand};

use super::{
    bidding::BiddingPanel, chat::Chat, connection_status::ConnectionStatus,
    hand::Hand, player::PlayerSeat, scoreboard::Scoreboard, trick::TrickArea,
    video_grid::VideoGrid,
};

#[component]
pub fn Table(session: GameSession) -> impl IntoView {
    let start_session = session.clone();
    let ready_session = session.clone();
    let leave_session = session.clone();
    let bidding_session = session.clone();
    let on_bid = Callback::new(move |command| {
        let _ = bidding_session.dispatch(command);
    });
    let status_session = session.clone();
    let start_view_session = session.clone();
    let bidding_view_session = session.clone();
    let hand_view_session = session.clone();
    let error_view_session = session.clone();

    view! {
        <div class="game-room">
            <header class="app-header">
                <div>
                    <h1>"BridgeRoom"</h1>
                    <span class="room-id">
                        "Room " {move || session.state.with(|state| state.room_id.clone())}
                    </span>
                </div>
                <div class="header-status">
                    <ConnectionStatus local=true />
                    <button class="secondary-button" on:click=move |_| {
                        let ready = ready_session.state.with(|state| {
                            state.player_at(state.my_seat).is_some_and(|player| !player.ready)
                        });
                        let _ = ready_session.dispatch(GameCommand::SetReady { ready });
                    }>
                        {move || if session.state.with(|state| state.player_at(state.my_seat).is_some_and(|player| player.ready)) { "Ready" } else { "Not ready" }}
                    </button>
                    <button class="secondary-button" on:click=move |_| {
                        let _ = leave_session.dispatch(GameCommand::LeaveGame);
                    }>
                        "Leave room"
                    </button>
                </div>
            </header>

            <div class="game-layout">
                <main class="game-board">
                    <section class="bridge-table" aria-label="Bridge table">
                        <div class=move || format!("seat {}", position_for(session.state.with(|state| state.my_seat), Seat::North))>
                            <PlayerSeat session=session.clone() seat=Seat::North />
                        </div>
                        <div class=move || format!("seat {}", position_for(session.state.with(|state| state.my_seat), Seat::West))>
                            <PlayerSeat session=session.clone() seat=Seat::West />
                        </div>
                        <div class="center-trick">
                            <TrickArea session=session.clone() />
                        </div>
                        <div class=move || format!("seat {}", position_for(session.state.with(|state| state.my_seat), Seat::East))>
                            <PlayerSeat session=session.clone() seat=Seat::East />
                        </div>
                        <div class=move || format!("seat {}", position_for(session.state.with(|state| state.my_seat), Seat::South))>
                            <PlayerSeat session=session.clone() seat=Seat::South />
                        </div>
                    </section>

                    <section class="game-controls">
                        <div class="game-status" role="status">{move || status_text(&status_session)}</div>
                        {move || if start_view_session.state.with(|state| matches!(state.phase, GamePhase::Lobby | GamePhase::HandComplete)) {
                            let label = start_view_session.state.with(|state| if state.phase == GamePhase::Lobby { "Start local game" } else { "Deal next hand" });
                            let click_session = start_session.clone();
                            view! { <button class="primary-button" on:click=move |_| { let _ = click_session.start_game(); }>{label}</button> }.into_any()
                        } else {
                            ().into_any()
                        }}
                        {move || if bidding_view_session.state.with(|state| state.phase == GamePhase::Bidding) {
                            view! {
                                <BiddingPanel
                                    on_bid=on_bid
                                    enabled=bidding_view_session.is_my_turn()
                                    can_double=bidding_view_session.state.with(|state| state.contract.is_some())
                                />
                            }.into_any()
                        } else {
                            ().into_any()
                        }}
                        {move || if hand_view_session.state.with(|state| state.phase != GamePhase::Lobby) {
                            view! { <Hand session=hand_view_session.clone() /> }.into_any()
                        } else {
                            ().into_any()
                        }}
                        {move || error_view_session.error.get().map(|message| view! {
                            <p class="error-message" role="alert">{message}</p>
                        })}
                    </section>
                </main>

                <aside class="sidebar">
                    <Scoreboard session=session.clone() />
                    <VideoGrid />
                    <Chat session=session.clone() />
                </aside>
            </div>
        </div>
    }
}

fn position_for(local: Seat, seat: Seat) -> &'static str {
    match (seat.index() + 4 - local.index()) % 4 {
        0 => "south",
        1 => "west",
        2 => "north",
        _ => "east",
    }
}

fn status_text(session: &GameSession) -> String {
    session.state.with(|state| match state.phase {
        GamePhase::Lobby => "Waiting for the local room to start".to_string(),
        GamePhase::Dealing => "Dealing cards…".to_string(),
        GamePhase::Bidding if state.is_my_turn() => {
            "Your turn to bid".to_string()
        }
        GamePhase::Bidding => {
            format!("{} to bid", state.current_turn.seat().label())
        }
        GamePhase::Playing if state.is_my_turn() => {
            "Your turn to play".to_string()
        }
        GamePhase::Playing => {
            format!("{} to play", state.current_turn.seat().label())
        }
        GamePhase::HandComplete => "Hand complete".to_string(),
        GamePhase::Complete => "Game finished".to_string(),
    })
}
