use super::*;

//状態
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Pause,
	Start,
	Play,
	Over,
}

//End of code.