use leptos::prelude::*;

use crate::components::{
    player::{PlayerSeat, Seat},
    trick::TrickArea,
};

#[component]
pub fn BridgeTable() -> impl IntoView {
    view! {
        <div class="bridge-table">

            <PlayerSeat
                seat=Seat::North
                name="North".to_string()
            />

            <PlayerSeat
                seat=Seat::West
                name="West".to_string()
            />

            <TrickArea/>

            <PlayerSeat
                seat=Seat::East
                name="East".to_string()
            />

            <PlayerSeat
                seat=Seat::South
                name="You".to_string()
            />

        </div>
    }
}