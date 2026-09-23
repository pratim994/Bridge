use leptos::prelude::*;

use crate::game::commands::{Bid, BidStrain, GameCommand};

#[component]
pub fn BiddingPanel(
    on_bid: Callback<GameCommand>,
) -> impl IntoView {
    let levels = (1..=7).collect::<Vec<_>>();

    view! {
        <div class="bidding-panel">

            <div class="bid-row special-bids">
                <button on:click=move |_| {
                    on_bid.run(GameCommand::PlaceBid {
                        bid: Bid::Pass,
                    });
                }>
                    "Pass"
                </button>

                <button>
                    "Double"
                </button>

                <button>
                    "Redouble"
                </button>
            </div>

            {levels.into_iter().map(|level| {
                view! {
                    <div class="bid-row">
                        {render_bid_button(
                            level,
                            BidStrain::Clubs,
                            on_bid
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::Diamonds,
                            on_bid
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::Hearts,
                            on_bid
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::Spades,
                            on_bid
                        )}

                        {render_bid_button(
                            level,
                            BidStrain::NoTrump,
                            on_bid
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
        <button on:click=move |_| {
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