# Turn Systems 

### Kind of Player Systems
**Player Centric**
Have the main game loop handle UI and call move_monsters() (or generally any other function that advances the whole world a turn) in the code for some of the player actions (the ones that consume a turn). Generally inflexible approach, although might seem appealing for animation-heavy and "modern" game.


### Some examples from other games:
#### Angband
['https://angband.readthedocs.io/en/latest/hacking/how-it-works.html']
**Main Features**
- Accumulation at baseline of a consumable resource (energy) simulataneously by all actor entities
- Multiplication of the baseline incrementation (speed)
- Varying costs for different actions (ex. most = 100, bow with extra shot = 50)
- Ordering of player's action before the enemies' action, and of enemies' action before the world's actions. 

In Angband, creatures act in order of “energy”, which roughly determines how many actions they can take per step through the simulation. The process_monsters() function in mon-move.c is responsible for walking through the list of all monsters in the current chunk (see the chunk) and having each monster act by calling process_monster(), which implements the highest level AI for monsters.

The process_player() function allows the player to act repeatedly until they do something that uses energy. Commands like looking around or inscribing items do not use energy; movement, attacking, casting spells, using items, and so on do. The rule of thumb is that a command that does not alter game engine state does not use energy, because it does not represent an action the character in the simulation is doing. The guts of the process_player() function are actually handled by process_command() in cmd-core.c, which looks up commands in the game_cmds table in that file.

At normal speed, you gain 10 points of energy every game turn. Once you have 100 energy points, you get to take an action. Most actions cost 100 energy units (the major exception being shooting an arrow or bolt with a bow of Extra Shots. This divides the energy cost by two - or, if it's a bow of Extra Shots +2, by three, thus allowing you to shoot several times before a monster gets an action).

Each extra speed point that you have gains you an extra energy point each game turn. Thus, if you have +10 to speed, you gain 20 energy points per game turn, twice as fast as normal. Normal monsters only get 10 energy points per game turn (same as the player), so you move twice as fast as they do. The slowing of gains from speed that occurs at about +28 or so to speed is then translated to each point of speed not giving a complete point of energy each game turn. It's the same effect, subjectively.

The main loop of the game, run_game_loop() is repeatedly called inside play_game(). Each iteration of the main loop is one “turn” in Angband parlance, or one step of the simulator. During each turn:

1. Player and monsters receive energy
2. All monsters with more energy than the player act
3. The player acts
4. All other monsters act
5. The UI updates
6. The world acts
7. End-of-turn housekeeping is done
    
