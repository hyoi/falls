//external modules
use bevy::{ prelude::*, diagnostic::*,};
use bevy_prototype_lyon::prelude::*;
use heron::*;
use rand::prelude::*;

//internal modules
mod types;
mod consts;
mod utils;

use types::*;
use consts::*;
use utils::*;

mod ui;
mod meteor;
// mod player;
// mod bg_stars;

use ui::*;
use meteor::*;
// use player::*;
// use bg_stars::*;

//メイン関数
fn main()
{	let main_window = WindowDescriptor
	{	title    : APP_TITLE.to_string(),
		width    : SCREEN_WIDTH,
		height   : SCREEN_HEIGHT,
		resizable: false,
		..Default::default()
	};
	
	let mut app = App::new();
	app
	//--------------------------------------------------------------------------------
	.insert_resource( main_window )							// メインウィンドウ
	.insert_resource( ClearColor( SCREEN_BGCOLOR ) )		// 背景色
	.insert_resource( Msaa { samples: 4 } )					// アンチエイリアス
	//--------------------------------------------------------------------------------
	.add_plugins( DefaultPlugins )							// デフォルトプラグイン
	.add_plugin( FrameTimeDiagnosticsPlugin::default() )	// fps計測のプラグイン
	.add_plugin( ShapePlugin )								// bevy_prototype_lyonを使う
	.add_plugin( PhysicsPlugin::default() )					// heronを使う
	//--------------------------------------------------------------------------------
	.add_state( GameState::Start )							// 状態遷移のState初期値
	.add_event::<GameState>()								// 状態遷移のEventキュー
	//--------------------------------------------------------------------------------
	.add_startup_system( spawn_camera )						// bevyのカメラ設置
	.add_system( handle_events_for_change_state )			// GameStateの変更
	//--------------------------------------------------------------------------------
	.add_plugin( PluginUi )									// UI
	.add_plugin( PluginFalls )								// 落下物
//	.add_plugin( PluginPlayer )								// 自機
//	.add_plugin( PluginBgStars )							// 背景の星空
	//--------------------------------------------------------------------------------
	.add_system( toggle_window_mode )						// [Alt]+[Enter]でフルスクリーン
	.add_system( handle_esc_key_for_pause )					// [Esc]でpause処理
	//--------------------------------------------------------------------------------
	;

	app.run();												// アプリの実行
}

//End of code.