use super::*;

//Pluginの手続き
pub struct PluginBGStars;
impl Plugin for PluginBGStars
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//--------------------------------------------------------------------------------
			.add_startup_system( initialize_bg_stars.system() )	// 初期の描画位置を決める
			.add_system( scroll_bg_stars.system() )				// 背景の星をスクロール
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//定義と定数

#[derive(Clone,Copy,Default,Debug)]
pub struct Star
{   xy : ( f32, f32 ),	//星の位置
	v  : ( f32, f32 ),	//星の速度
	r  : f32,			//星の半径
	hue: f32,			//星の色相
}
pub struct BgStars ( pub Vec::<Star> );	//星の管理用リスト
struct BgStarsTimer ( Timer );			//星の発生タイマー

const X_MIN: f32 = SCREEN_WIDTH  / -2.0;
const X_MAX: f32 = SCREEN_WIDTH  /  2.0;
const Y_MIN: f32 = SCREEN_HEIGHT / -2.0;
const Y_MAX: f32 = SCREEN_HEIGHT /  2.0;

////////////////////////////////////////////////////////////////////////////////

//背景の星の初期表示位置を決める
fn initialize_bg_stars( mut cmds: Commands )
{	let mut rng = rand::thread_rng();
	let mut bg_stars = Vec::with_capacity( 400 ); //実測で最大360個くらい？

	( 0..300 ).for_each( |_|
	{	let xy  = ( rng.gen_range( X_MIN..X_MAX ), rng.gen_range( Y_MIN..Y_MAX ) );
		let r   = rng.gen_range( 0.2..=1.5);
		let vy  = if r > 1.0 { -150.0 } else if r > 0.5 { -100.0 } else { -50.0 };
		let v   = ( 0.0, vy );
		let hue = rng.gen_range( 0.0..360.0 );

		bg_stars.push( Star { xy, v, r, hue } );
	} );

	cmds.insert_resource( BgStars ( bg_stars ) );
	cmds.insert_resource( BgStarsTimer ( Timer::from_seconds( 0.02, false ) ) );
}

//背景の星をスクロールする
fn scroll_bg_stars
(	mut bg_stars: ResMut<BgStars>,
	mut bg_timer: ResMut<BgStarsTimer>,
	mut canvas: ResMut<Canvas>,
	time: Res<Time>,
)
{	//前回からの経過時間
	let time_delta = time.delta().as_secs_f32();

	//タイマーで調節したタイミングで、画面上端に新しい星を発生させる
	if bg_timer.0.tick( time.delta() ).finished()
	{	bg_timer.0.reset();

		let mut rng = rand::thread_rng();
		let xy  = ( rng.gen_range( X_MIN..X_MAX ), Y_MAX );
		let r   = rng.gen_range( 0.2..=1.5);
		let vy  = if r > 1.0 { -150.0 } else if r > 0.5 { -100.0 } else { -50.0 };
		let v   = ( 0.0, vy );
		let hue = rng.gen_range( 0.0..360.0 );

		bg_stars.0.push( Star { xy, v, r, hue } );
	}

	//スクロール位置を計算してcanvasに星を描画する
	bg_stars.0.iter_mut().for_each( | star |
	{	let (  x,  y ) = star.xy;
		let ( vx, vy ) = star.v;
		let new_x = x + vx * time_delta;
		let new_y = y + vy * time_delta;
		( *star ).xy = ( new_x, new_y );

		let mut hue = ( *star ).hue + 360.0 * time_delta;
		if hue > 360.0 { hue %= 360.0 }
		( *star ).hue = hue;
		let ( saturation, lightness, alpha ) = ( 1.0, 0.6, 1.0 );

		let center = Vec2::new( new_x, new_y );
		let radius = star.r;
		let circle = Circle { center, radius };
		let dmode  = bevy_canvas::DrawMode::fill_simple(); //名前がbevy_prototype_lyonとバッティングした
		let color  = Color::Hsla { hue, saturation, lightness, alpha };

		canvas.draw( &circle, dmode, color );
	} );

	//画面外に出た星の情報を削除する
	bg_stars.0.retain( | star | { let ( _, y ) = star.xy; y > Y_MIN } );
}

//End of code