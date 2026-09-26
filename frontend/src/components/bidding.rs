use leptos::prelude::*;

use crate::game::commands::{Bid, BidStrain, GameCommand};

#[component]
pub fn BiddingPanel(
    on_bid: Callback<GameCommand>,
    #[prop(default = true)] enabled: bool,
    #[prop(default = false)] can_double: bool,
) -> impl IntoView {
    let levels = (1..=7).collect::<Vec<_>>();

    view! {
        <div class="bidding-panel">
            <h2>"Auction"</h2>

            <div class="bid-row special-bids">
                <button disabled=!enabled on:click=move |_| {
                    on_bid.run(GameCommand::PlaceBid {
                        bid: Bid::Pass,
                    });
                }>
                    "Pass"
                </button>

                <button disabled=!(enabled && can_double) on:click=move |_| {
                    on_bid.run(GameCommand::PlaceBid { bid: Bid::Double });
                }>
                    "Double"
                </button>

                <button disabled=!(enabled && can_double) on:click=move |_| {
                    on_bid.run(GameCommand::PlaceBid { bid: Bid::Redouble });
                }>
                    "Redouble"
                </button>
            </div>

            {levels.into_iter().map(|level| {
                view! {
                    <div class="bid-row">
                        {render_bid_button(
                            level,
                            BidStrain::Clubs,
                            on_bid,
                            enabled
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::Diamonds,
                            on_bid,
                            enabled
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::Hearts,
                            on_bid,
                            enabled
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::Spades,
                            on_bid,
                            enabled
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::NoTrump,
                            on_bid,
                            enabled
                        )}
                    </div>
                }
            }).collect_view()}

        </div>
    }
}

fn render_bid_button(
    level: u8,
    strain: BidStrain,
    on_bid: Callback<GameCommand>,
    enabled: bool,
) -> impl IntoView {
    let label = format!(
        "{}{}",
        level,
        match strain {
            BidStrain::Clubs => "♣",
            BidStrain::Diamonds => "♦",
            BidStrain::Hearts => "♥",
            BidStrain::Spades => "♠",
            BidStrain::NoTrump => "NT",
        }
    );

    view! {
        <button disabled=!enabled on:click=move |_| {
            on_bid.run(GameCommand::PlaceBid {
                bid: Bid::Contract {
                    level,
                    strain,
                },
            });
        }>
            {label}
        </button>
    }
}
