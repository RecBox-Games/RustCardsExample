# RustCardsExample
This example is meant to show how a GameNite game can be developed in Rust with 
ggez and tested natively. It also provides a basic framework for implementing a 
card game using a standard deck of cards.


## Instructions

- run `setup.sh` if you haven't already
- run `build_run.sh` to start the game and controlpad server
- point the browser on this machine at `localhost:3000`
- OR use the qr code printed in the terminal to connect a phone on the same 
  wifi network


## Explanation

### Game
The game is written in Rust using the ggez graphics libary. The code is in `src/` 
and is somewhat commented. 

- `main.rs` contains the game loop which calls `MyCardGame`'s `draw()` and `update()` 
  functions and keeps track of received messages from controlpads, calling 
  `handle_controlpad_message()` on them.

- `my_card_game.rs` contains the state and logic of the game

- `draw_my_card_game.rs` contains the representation of the state of the game 
  and specifies how everything should be drawn on screen

- `progress.rs` contains the `Progression` struct which is used to keep track 
  of movements of cards over time

- `standard_deck.rs` contains structs and enums to represent a standard deck of 
  cards and functionality to load the deck images from `resources/`


### Controller
The phones are controllers but we refer to the web-based instance of client 
code on the phone as a controlpad. That code exists in `controller/` and is 
served by `controlpad_server/controlpad_web_server.js` over LAN


### Communication between Game and Controlpads
Communication progresses  `game` -> `controlpad_server` -> `controlpad clients`
in one direction and `controlpad clients` -> `controlpad_server` -> `game` in 
the other direction. Communication between `game` <-> `controlpad_server` 
happens via ipc (file io in this case), and communication between 
`controlpad_server` <-> `controlpad clients` happens via websockets.

- read `protocol.md` for a description of the specific messages sent between 
  the game and the controlpads
