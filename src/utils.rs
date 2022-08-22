use super::*;

//bevyのカメラの設置
pub fn spawn_camera( mut cmds: Commands )
{	cmds.spawn_bundle( Camera2dBundle::default() );
}

//[Alt]+[Enter]でウィンドウとフルスクリーンを切り替える
#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_window_mode( inkey: Res<Input<KeyCode>>, mut window: ResMut<Windows> )
{	use KeyCode::*;
	let is_alt = inkey.pressed( LAlt ) || inkey.pressed( RAlt );
	let is_alt_return = is_alt && inkey.just_pressed( Return );

	if is_alt_return
	{	use bevy::window::WindowMode::*;
		if let Some( window ) = window.get_primary_mut()
		{	let mode = if window.mode() == Windowed { Fullscreen } else { Windowed };
			window.set_mode( mode );
		}
	}
}

//[Esc]が入力さたらPauseする
pub fn handle_esc_key_for_pause
(	mut q: Query<&mut Visibility, With<MessagePause>>,
	mut phy_timer: ResMut<PhysicsTime>,
	mut state: ResMut<State<GameState>>,
	mut inkey: ResMut<Input<KeyCode>>,
)
{	if ! inkey.just_pressed( KeyCode::Escape ) { return }

	let now = *state.current();
	if now != GameState::Pause && now != GameState::Play { return }

	if let Ok( mut ui ) = q.get_single_mut()
	{	match now
		{	GameState::Pause =>
			{	ui.is_visible = false;
				state.pop().unwrap();
				phy_timer.resume();
			}
			_ => 
			{	ui.is_visible = true;
				state.push( GameState::Pause ).unwrap();
				phy_timer.pause();
			}
		}
		inkey.reset( KeyCode::Escape ); // https://bevy-cheatbook.github.io/programming/states.html#with-input
	}
}

//eventで渡されたstateへ遷移する(キューの先頭だけ処理)
pub fn handle_events_for_change_state
(	mut events: EventReader<GameState>,
	mut state : ResMut<State<GameState>>,
)
{	if let Some( next ) = events.iter().next()
	{	let _ = state.overwrite_set( *next ); // 戻り値をletで受けないとワーニング
	}
}

//End of code.