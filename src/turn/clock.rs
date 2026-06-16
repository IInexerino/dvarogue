use bevy::{ecs::resource::Resource, prelude::{Deref, DerefMut}};


/// Total turn counter that runs from the start to the end of a game.
/// 
/// Each 'average' turn is 100, this is kept as an int, 
/// but is usually displayed to the player as a string with one or two decimal points.
/// 
/// It does not advance by ticking `+= 1`, but instead advances to the scheduled action time 
/// of the soonest acting `Actor` entities, and is updated so.
#[derive(Resource, Default, Deref, DerefMut)] 
pub struct Clock(pub u64);

impl Clock {
    pub fn to_decimal_string(&self) -> String {
        let s = self.to_string();
        // Possible Bug? ; Could this possibly return more or less 
        let len = s.len();

        if len == 1 {
            return String::from("0.0")
        } else if len == 2 {
            return format!("0.{}", &s[len-2..len-1]) 
        } else {
            return format!("{}.{}", &s[..len-2], &s[len-2..len-1])
        }
    }
}