use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut tmin = min;
        let mut tmax = max;
        let mut guess = ((tmax-tmin)/2) + tmin;
        loop {
            let value = player.ask_to_compare(guess);
            if value == 0{
                return guess;
            }
            else if value == 1{ // num is greater
                tmin = guess;
                
            }
            else if value == -1{ // num is less
                tmax = guess;
                
            }
             guess = ((tmax-tmin)/2) + tmin; //set the next guess to the new middle value
            
        } 
    }
}
