
# Bridge Rules — Frontend Reference

## Purpose

This document defines the bridge concepts that the frontend needs to understand for presentation and interaction.

The authoritative implementation of bridge rules belongs to the backend/game engine.

## Players

A standard bridge game contains four players.

Seats:

- North
- East
- South
- West

Each player normally receives 13 cards.

## Partnerships

Standard partnerships are:

- North/South
- East/West

The frontend should derive partnership presentation from seat/game state rather than hard-coding player names.

## Cards

A standard deck contains:

- Clubs
- Diamonds
- Hearts
- Spades

and ranks:

- 2 through 10
- Jack
- Queen
- King
- Ace

## Bidding

The bidding phase consists of calls made in turn.

Calls include:

- Pass
- contract bids
- Double
- Redouble

The frontend presents legal controls based on authoritative state.

The backend decides whether a call is legal.

## Trick Play

During play:

- one player leads;
- subsequent players play in turn;
- players must follow suit when required;
- the highest applicable card wins the trick;
- trump affects trick resolution when a trump suit exists.

The frontend displays the trick and its participants.

The backend determines legality and winner.

## Scoring

Scoring is authoritative backend logic.

The frontend displays score information supplied by the server.

## Cheating Prevention

The frontend must never be relied upon to enforce hidden information or game legality.
