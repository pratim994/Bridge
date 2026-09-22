use leptos::prelude::*;

use crate::game::{PlayedCard, Seat};

use super::card::CardView;

#[component]
pub fn TrickView(
    trick: Vec<PlayedCard>,
) -> impl IntoView {
    view! {
        <div class="trick">

            {trick
                .into_iter()
                .map(|played| {
                    let position = match player_seat(&played.player_id) {
                        Seat::North => "trick-card north-card",
                        Seat::East => "trick-card east-card",
                        Seat::South => "trick-card south-card",
                        Seat::West => "trick-card west-card",
                    };

                    view! {
                        <div class=position>
                            <CardView
                                card=played.card
                                playable=false
                            />
                        </div>
                    }
                })
                .collect_view()
            }

        </div>
    }
}

fn player_seat(player_id: &str) -> Seat {
    match player_id {
        "north" => Seat::North,
        "east" => Seat::East,
        "south" => Seat::South,
        "west" => Seat::West,
        _ => Seat::South,
    }
}