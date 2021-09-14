# Program Structure
Here we write our programs structure
### Setup board and peices
- Create the actuall board
- Load starting position as FEN string
- Load Pieces


### Gameloop



# Program parts
Here we write the different parts/modules of the program.

Should the Peices handle Promotion or the Rules(I think the rules should handle promotion and castling but the peices [pawns] should handle passant and en passant)


## Different modules
#### Peices
- Moves
  - Legal moves
    - Long moves
    - Fixed moves
  - Illegal moves (maybe Rules should handle this)
  
#### Rules
- Uses Peices
- Starting Position FEN
- Castle
- Check (& Mate)
- Promotion

  
#### Board
- Uses Rules
- Handles peices (Stores and changes board)
- Handle Score

#### Game
- uses board
- makes moves
- checks rules
  
#### Client Interface
- Handles user input
- Promts illegal moves
- Highlighs check
  
# Notes
Here we add notes