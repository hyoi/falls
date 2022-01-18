use super::*;

//状態
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
pub enum GameState
{	Pause,
	Start,
	Play,
	Over,
}

//Component
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LifeGauge;

#[derive(Component)]
pub struct Meteor;

//Resource
pub struct FallingRhythm { pub timer: Timer }	//落下物の発生タイマー
pub struct InfoNumOfFalls { pub count: usize }	//落下中の数

//Resource
pub struct LifeTime { pub time: f64 }
pub struct CollisionDamage { pub life: f32 }

//End of code.