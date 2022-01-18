use super::*;

//Pluginの手続き
pub struct PluginBgStars;
impl Plugin for PluginBgStars
{	fn build( &self, app: &mut App )
	{	app
		//--------------------------------------------------------------------------------
			.add_startup_system( initialize_bg_stars )		// 背景の星の初期状態を決める
			.add_system( scroll_bg_stars )					// 背景の星をスクロール
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//定義と定数

//描画範囲
const LEFT  : f32 = SCREEN_WIDTH  / -2.0;
const RIGHT : f32 = SCREEN_WIDTH  /  2.0;
const TOP   : f32 = SCREEN_HEIGHT /  2.0;
const BOTTOM: f32 = SCREEN_HEIGHT / -2.0;

////////////////////////////////////////////////////////////////////////////////

//背景の星の位置と速度、大きさ、色を乱数で決める
fn generate_background_star( top: bool ) -> Star
{	let mut rng = rand::thread_rng();

	let xy  = ( rng.gen_range( LEFT..RIGHT ), if top { TOP } else { rng.gen_range( BOTTOM..TOP ) } );
	let r   = rng.gen_range( 0.2..=1.5);
	let v   = ( 0.0, if r > 1.0 { -150.0 } else if r > 0.5 { -100.0 } else { -50.0 } );
	let hue = rng.gen_range( 0.0..360.0 );

	Star { xy, v, r, hue }
}

//背景の星の初期状態を決める
fn initialize_bg_stars( mut cmds: Commands )
{	let mut stars = Vec::with_capacity( 400 ); //実測で最大360個くらい？
	( 0..300 ).for_each( |_| stars.push( generate_background_star( false ) ) );
	let timer = Timer::from_seconds( 0.02, false );

	cmds.insert_resource( BgStars { stars, timer } );
}

//背景の星をスクロールする
fn scroll_bg_stars
(	mut bg: ResMut<BgStars>,
	mut canvas: ResMut<Canvas>,
	time: Res<Time>,
)
{	//前回からの経過時間
	let time_delta = time.delta().as_secs_f32();

	//タイマーで調節したタイミングで、画面上端に新しい星を発生させる
	if bg.timer.tick( time.delta() ).finished()
	{	bg.timer.reset();
		bg.stars.push( generate_background_star( true ) ); //trueはY軸を上端固定にする指示
	}

	//スクロール位置を計算してcanvasに星を描画する
	bg.stars.iter_mut().for_each( | star |
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
	bg.stars.retain( | star | { let ( _, y ) = star.xy; y > BOTTOM } );
}

//End of code