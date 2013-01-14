use std;

import comm::{listen, methods};
import task::spawn;
import iter::repeat;
import rand::{seeded_rng, seed};
import uint::range;
import io::println;

fn main() {
    // Open a channel to receive game results
    do listen |result_from_game| {

        let times = 10;
        let player1 = ~"graydon";
        let player2 = ~"patrick";

        for repeat(times) {
            // Start another task to play the game
            do spawn |copy player1, copy player2| {
                let outcome = play_game(player1, player2);
                result_from_game.send(outcome);
            }
        }

        // Report the results as the games complete
        for range(0, times) |round| {
            let winner = result_from_game.recv();
            println(#fmt("%s wins round #%u", winner, round));
        }
    }

    fn play_game(player1: ~str, player2: ~str) -> ~str {

        // Our rock/paper/scissors types
        enum gesture {
            rock, paper, scissors
        }

        let rng = seeded_rng(seed());
        // A small inline function for picking an RPS gesture
        let pick = || (~[rock, paper, scissors])[rng.gen_uint() % 3];

        // Pick two gestures and decide the result
        alt (pick(), pick()) {
            (rock, scissors) | (paper, rock) | (scissors, paper) { copy player1 }
            (scissors, rock) | (rock, paper) | (paper, scissors) { copy player2 }
            _ { ~"tie" }
        }
    }
}