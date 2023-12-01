# RustCardsExample
This example is meant to show how a GameNite game can be developed in Rust with 
ggez and tested natively. It also provides a basic framework for implementing a 
card game using a standard deck of cards.

#### Note
You need to have access to the private **ControlpadServer** repo in order to 
build this (which requires that you've signed a code non-disclosure agreement)
- If there is interest, we can put together a pre-built library which will
  allow you to use this repository without access to **ControlpadServer**


## Instructions

- run `setup.sh` if you haven't already
- run `build_run.sh` to start the game and controlpad server
- point the browser on this machine at `localhost:3000`
- OR use the qr code printed in the terminal to connect a phone on the same 
  wifi network

#### Multiple Controllers

- if you open two tabs to localhost:3000 on the same machine, the controlpad 
  server will treat them both as the same client
- to have your two tabs treated like two separate clients (presumably for 
  testing purposes), add ?subid=N to the url, where N is a nonzero number 
  unique to the tab
  - e.g. tab1: `localhost:3000`, tab2: `localhost:3000?subid=1`, tab3: `localhost:3000?subid=2`, 


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

- `resources.rs` contains the `GameResources` struct which contains graphical 
  resources (images and text renders) and is passed to `draw()` functions where 
  they are used

- `standard_deck.rs` contains structs and enums to represent a standard deck of 
  cards and functionality to load the deck images from `resources/`


### Controller
The phones are controllers but we refer to the web-based instance of client 
code on the phone as a controlpad. That code exists in `controller/` and is 
served by `controlpad_server/controlpad_web_server.js` over LAN

- `controlpads.js` has the code for handling the websocket

- `app.js` is what the game developer is meant to implement

### Communication between Game and Controlpads
Communication progresses  `game` -> `controlpad_server` -> `controlpad clients`
in one direction and `controlpad clients` -> `controlpad_server` -> `game` in 
the other direction. Communication between `game` <-> `controlpad_server` 
happens via ipc (file io in this case), and communication between 
`controlpad_server` <-> `controlpad clients` happens via websockets.

- read `protocol.md` for a description of the specific messages sent between 
  the game and the controlpads
