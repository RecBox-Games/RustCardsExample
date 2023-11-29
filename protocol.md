### Legend / Special Character
`<>` encloses a variable
`{}` encloses an optional
`|`  separates the options within an optional


## Controlpads to Game
`state-request`
    - Sent by controlpad when it doesn't know what it's current state is supposed 
      to be. Game must respond with a state message.

`deal` 
    - Sent when 'Deal' button is pressed. Causes game to transfer a card from the 
      deck to the splayed area.

`card:{L|R},<suit>,<rank>`
    - Sent when a card is pressed. Causes the center card in the game to be 
      replaced by the specified card.


## Game to Controlpads
`state:<Lsuit>,<Lrank>:<Rsuit>,<Rrank>`
    - Sent in response to a state request. Causes the controller to be updated 
      to reflect the given state.
