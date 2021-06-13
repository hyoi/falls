//external modules
use bevy::{ prelude::*, diagnostic::*,};
use bevy_canvas::{ *, common_shapes::*,};
use bevy_prototype_lyon::prelude::*;
use heron::*;
use rand::prelude::*;

//internal modules
mod bg_stars;
mod player;
mod meteor;
mod ui;

use bg_stars::*;
use player::*;
use meteor::*;
use ui::*;

////////////////////////////////////////////////////////////////////////////////

//アプリのTitle
const APP_TITLE: &str = "落下物";

//マップの縦横のグリッド数（今回はあまり意味なし）
const MAP_WIDTH : usize = 25;
const MAP_HEIGHT: usize = 40;

//表示倍率、ウィンドウの縦横pixel数と背景色
const SCREEN_SCALING: usize = 3;
const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * MAP_HEIGHT as f32;
const SCREEN_BGCOLOR: Color = Color::BLACK;

////////////////////////////////////////////////////////////////////////////////

//状態
#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
enum GameState
{	Pause,
	Start,
	Play,
	Over,
}

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{	let main_window = WindowDescriptor
	{	title    : APP_TITLE.to_string(),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..Default::default()
	};
	
	App::build()
	//--------------------------------------------------------------------------------
		.insert_resource( main_window )							// メインウィンドウ
		.insert_resource( ClearColor( SCREEN_BGCOLOR ) )		// 背景色
	//	.insert_resource( Msaa { samples: 4 } )					// アンチエイリアス(有効にするとbevy_canvasがダメだった)
	//--------------------------------------------------------------------------------
		.add_plugins( DefaultPlugins )							// デフォルトプラグイン
		.add_plugin( FrameTimeDiagnosticsPlugin::default() )	// fps計測のプラグイン
		.add_plugin( CanvasPlugin )								// bevy_canvasを使う
		.add_plugin( ShapePlugin )								// bevy_prototype_lyonを使う
		.add_plugin( PhysicsPlugin::default() )					// heronを使う
	//--------------------------------------------------------------------------------
		.add_state( GameState::Start )							// 状態遷移のState初期値
		.add_event::<GameState>()								// 状態遷移のEventキュー
	//--------------------------------------------------------------------------------
		.add_startup_system( spawn_camera.system() )			// bevyのカメラ設置
		.add_system( handle_events_for_change_state.system() )	// GameStateの変更
	//--------------------------------------------------------------------------------
		.add_plugin( PluginPlayer )								// 自機
		.add_plugin( PluginFalls )								// 落下物
		.add_plugin( PluginBgStars )							// 背景の星空
		.add_plugin( PluginUi )									// UI
	//--------------------------------------------------------------------------------
		.add_system( toggle_window_mode.system() )				// [Alt]+[Enter]でフルスクリーン
		.add_system( handle_esc_key_for_pause.system() )		// [Esc]でpause処理
	//--------------------------------------------------------------------------------
	.run();														// アプリの実行
}

////////////////////////////////////////////////////////////////////////////////

//bevyのカメラの設置
fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( UiCameraBundle::default() );
	cmds.spawn_bundle( OrthographicCameraBundle::new_2d() );
}

////////////////////////////////////////////////////////////////////////////////

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
fn toggle_window_mode( inkey: Res<Input<KeyCode>>, mut window: ResMut<Windows> )
{	use KeyCode::*;
	let is_alt = inkey.pressed( LAlt ) || inkey.pressed( RAlt );
	let is_alt_return = is_alt && inkey.just_pressed( Return );

	if is_alt_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let mode = if window.mode() == Windowed { Fullscreen { use_size: true } } else { Windowed };
			window.set_mode( mode );
		}
	}
}

//[Esc]が入力さたらPauseする
fn handle_esc_key_for_pause
(	mut q_ui : Query<&mut Visible, With<MessagePause>>,
	mut phy_timer: ResMut<PhysicsTime>,
	mut state: ResMut<State<GameState>>,
	mut inkey: ResMut<Input<KeyCode>>,
)
{	let now = *state.current();
	if now != GameState::Pause && now != GameState::Play { return }

	let pause_key = KeyCode::Escape;
	if let Ok( mut ui ) = q_ui.single_mut()
	{	if inkey.just_pressed( pause_key ) 
		{	if now == GameState::Pause
			{	ui.is_visible = false;
				state.pop().unwrap();
				phy_timer.resume();
			}
			else
			{	ui.is_visible = true;
				state.push( GameState::Pause ).unwrap();
				phy_timer.pause();
			}
			inkey.reset( pause_key ); // https://bevy-cheatbook.github.io/programming/states.html#with-input
		}
	}
}

////////////////////////////////////////////////////////////////////////////////

//eventで渡されたstateへ遷移する(キューの先頭だけ処理)
fn handle_events_for_change_state
(	mut events: EventReader<GameState>,
	mut state : ResMut<State<GameState>>,
)
{	if let Some( next ) = events.iter().next()
	{	let _ = state.overwrite_set( *next ); // 戻り値をletで受けないとワーニング
	}
}

//End of code.