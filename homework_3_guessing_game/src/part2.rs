use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32,  mut max: u32) -> u32 {
        let mut guess = ((max-min)/2) + min;
        for  i in min..max{
            let value = player.ask_to_compare(guess);
            if value == 0{
                return guess;
            }
            else if value == 1{ // num is greater
                min = guess;
                
            }
            else if value == -1{ // num is less
                max = guess;
                
            }
             guess = ((max-min)/2) + min; //set the next guess to the new middle value
            
        } 
        return 0;
    }
}
