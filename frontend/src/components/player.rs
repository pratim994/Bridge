use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    North,
    East,
    South,
    West,
}

impl Seat {
    pub fn label(self) -> &'static str {
        match self {
            Seat::North => "North",
            Seat::East => "East",
            Seat::South => "South",
            Seat::West => "West",
        }
    }
}

#[component]
pub fn PlayerSeat(
    seat: Seat,
    name: String,
) -> impl IntoView {
    view! {
        <div class=format!(
            "player-seat player-{}",
            seat.label().to_lowercase()
        )>
            <div class="player-name">
                {name}
            </div>
        </div>
    }
}
    