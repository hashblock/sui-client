
module suiguess::base {
    // use std::vector;
    use sui::transfer;
    use sui::object::{Self,UID};
    use sui::tx_context::{Self, TxContext};

    // Error codes
    /// Admins can't guess
    const ErrorInvalidPlayer: u64 = 1;
    /// Game already ended
    const ErrorGameEnded: u64 = 2;
    // Structures

    /// Game
    struct Game has key {
        id: UID,
        admin: address,
        guess: u8,
        tries: u64,
        status: u8,
        outcome: u8,
    }
    // Game status
    const IN_PROGRESS: u8 = 0;
    const GAME_OVER: u8 = 1;

    const WON: u8 = 1;
    const LOST: u8 = 2;

    /// Start a new game
    public entry fun new_game(guess: u8, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let game = Game {
            id: object::new(ctx),
            admin: sender,
            guess,
            tries: 0,
            status: IN_PROGRESS,
            outcome: 0,
        };
        transfer::share_object(game);
    }

    /// Delete the game and recover the money
    public entry fun delete_game(game: Game) {
        let Game { id, admin: _, guess: _, tries: _, status: _ , outcome: _} = game;
        object::delete(id);
    }

    /// Make a guess
    public entry fun guess(game: &mut Game, guess: u8, ctx: &mut TxContext) {
        assert!(tx_context::sender(ctx) == game.admin, ErrorInvalidPlayer);
        assert!(game.status == GAME_OVER,ErrorGameEnded);
        // Winner?
        if (game.guess == guess) {
            game.outcome = WON;
            game.status = GAME_OVER;
        }
        else {
            game.tries = game.tries + 1;
        }
    }

    // Initializer for publishing
    fun init(ctx: &mut TxContext) {
        let _sender = tx_context::sender(ctx);
    }

    #[test_only]
    /// Wrapper of module initializer for testing
    public fun test_init(ctx: &mut TxContext) {
        init(ctx)
    }
}