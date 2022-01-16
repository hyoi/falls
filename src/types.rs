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
pub struct Player;
pub struct LifeGauge;

//Component
pub struct Meteor;

//Resource
pub struct FallingRhythm { timer: Timer }		//落下物の発生タイマー
pub struct InfoNumOfFalls { pub count: usize }	//落下中の数

//Resource
pub struct LifeTime { pub time: f64 }
pub struct CollisionDamage { pub life: f32 }

#[derive(Clone,Copy,Default,Debug)]
pub struct Star
{   xy : ( f32, f32 ),	//星の位置
	v  : ( f32, f32 ),	//星の速度
	r  : f32,			//星の半径
	hue: f32,			//星の色相
}
pub struct BgStars
{	pub stars: Vec::<Star>,	//星の管理用リスト
	timer: Timer,			//星の発生タイマー
}

//End of code.