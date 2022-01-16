use super::*;

//Pluginの手続き
pub struct PluginUi;
impl Plugin for PluginUi
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//--------------------------------------------------------------------------------
			.add_startup_system( spawn_text_ui_message.system() )	// Text UIを生成
		//--------------------------------------------------------------------------------
			.add_system( update_header_ui_left.system() )			// 情報を更新
			.add_system( update_header_ui_right.system() )			// 情報を更新
			.add_system( update_footer_ui_left.system() )			// 情報を更新
			.add_system( update_footer_ui_center.system() )			// 情報を更新
			.add_system( update_footer_ui_right.system() )			// 情報を更新
		//--------------------------------------------------------------------------------
			.add_system_set											// GameState::Start
			(	SystemSet::on_enter( GameState::Start )				// on_enter()
				.with_system( show_start_message.system() )			// STARTメッセージ表示
			)
			.add_system_set											// GameState::Start
			(	SystemSet::on_update( GameState::Start )			// on_update()
				.with_system( handle_input_space_key.system() )		// キー入力待ち
			)
			.add_system_set											// GameState::Start
			(	SystemSet::on_exit( GameState::Start )				// on_exit()
				.with_system( hide_start_message.system() )			// STARTメッセージ非表示
			)
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut pause_text = text_messsage( &MESSAGE_PAUSE, &asset_svr );
	let mut start_text = text_messsage( &MESSAGE_START, &asset_svr );
	let mut over_text  = text_messsage( &MESSAGE_OVER , &asset_svr );
	pause_text.visible.is_visible = false;	//初期は非表示
	start_text.visible.is_visible = false;	//初期は非表示
	over_text.visible.is_visible  = false;	//初期は非表示

	//上端に表示するtext
	let mut header_ui_left   = text_messsage( &HEADER_UI_LEFT  , &asset_svr );
	let mut header_ui_right  = text_messsage( &HEADER_UI_RIGHT , &asset_svr );
	header_ui_left.style.align_self = AlignSelf::FlexStart;
	header_ui_left.text.alignment.horizontal = HorizontalAlign::Left;
	header_ui_right.style.align_self = AlignSelf::FlexEnd;
	header_ui_right.text.alignment.horizontal = HorizontalAlign::Right;

	//下端に表示するtext
	let mut footer_ui_left   = text_messsage( &FOOTER_UI_LEFT  , &asset_svr );
	let mut footer_ui_center = text_messsage( &FOOTER_UI_CENTER, &asset_svr );
	let mut footer_ui_right  = text_messsage( &FOOTER_UI_RIGHT , &asset_svr );
	footer_ui_left.style.align_self = AlignSelf::FlexStart;
	footer_ui_left.text.alignment.horizontal = HorizontalAlign::Left;
	footer_ui_center.style.align_self = AlignSelf::Center;
	footer_ui_center.text.alignment.horizontal = HorizontalAlign::Center;
	footer_ui_right.style.align_self = AlignSelf::FlexEnd;
	footer_ui_right.text.alignment.horizontal = HorizontalAlign::Right;

	//隠しフレームの上に子要素を作成する
	cmds.spawn_bundle( hidden_frame_for_centering() ).with_children( | cmds |
	{	cmds.spawn_bundle( pause_text ).insert( MessagePause );
		cmds.spawn_bundle( start_text ).insert( MessageStart );
		cmds.spawn_bundle( over_text  ).insert( MessageOver  );

		cmds.spawn_bundle( hidden_header_frame() ).with_children( | cmds |
		{	cmds.spawn_bundle( header_ui_left   ).insert( HeaderUiLeft   );
			cmds.spawn_bundle( header_ui_right  ).insert( HeaderUiRight  );
		} );

		cmds.spawn_bundle( hidden_footer_frame() ).with_children( | cmds |
		{	cmds.spawn_bundle( footer_ui_left   ).insert( FooterUiLeft   );
			cmds.spawn_bundle( footer_ui_center ).insert( FooterUiCenter );
			cmds.spawn_bundle( footer_ui_right  ).insert( FooterUiRight  );
		} );
	} );
}

//TextBundleを作る
fn text_messsage( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
{	let mut sections = Vec::new();
	for ( value, file, size, color ) in message.iter()
	{	let value = value.to_string();
		let style = TextStyle
		{	font     : asset_svr.load( *file ),
			font_size: *size,
			color    : *color
		};
		sections.push( TextSection { value, style } );
	}
	let alignment = TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center };
	let text = Text { sections, alignment };
	let style = Style { position_type: PositionType::Absolute, ..Default::default() };
	TextBundle { style, text, ..Default::default() }
}

//中央寄せ用の隠しフレーム
fn hidden_frame_for_centering() -> NodeBundle
{	let per100 = Val::Percent( 100.0 );
	let style = Style
	{	size: Size::new( per100, per100 ),
		position_type  : PositionType::Absolute,
		justify_content: JustifyContent::Center,
		align_items    : AlignItems::Center,
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//上端幅合せ用の隠しフレーム
fn hidden_header_frame() -> NodeBundle
{	let width  = Val::Px( SCREEN_WIDTH  );
	let height = Val::Px( SCREEN_HEIGHT );
	let style = Style
	{	size: Size::new( width, height ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexEnd, //画面の上端
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//下端幅合せ用の隠しフレーム
fn hidden_footer_frame() -> NodeBundle
{	let width  = Val::Px( SCREEN_WIDTH  );
	let height = Val::Px( SCREEN_HEIGHT );
	let style = Style
	{	size: Size::new( width, height ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexStart, //画面の下端
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

////////////////////////////////////////////////////////////////////////////////

//上端の情報表示を更新する(左)
fn update_header_ui_left
(	mut q: Query<&mut Text, With<HeaderUiLeft>>,
	o_collsion: Option<Res<CollisionDamage>>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	let life_gauge = match o_collsion
		{	Some( collision ) => format!( "{:03}", collision.life.max( 0.0 ) ),
			None              => NA_STR3.to_string()
		};
		ui.sections[ 1 ].value = life_gauge;
	}
}

//上端の情報表示を更新する(右)
fn update_header_ui_right
(	mut q: Query<&mut Text, With<HeaderUiRight>>,
	o_life: Option<Res<LifeTime>>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	let life_time = match o_life
		{	Some( life ) => format!( "{:2.2}", life.time ),
			None         => NA_TIME.to_string()
		};
		ui.sections[ 1 ].value = life_time;
	}
}

//下端の情報表示を更新する(左)
fn update_footer_ui_left
(	mut q: Query<&mut Text, With<FooterUiLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	let fps_avr = if let Some( fps ) = diag.get( FrameTimeDiagnosticsPlugin::FPS )
		{	match fps.average()
			{	Some( avg ) => format!( "{:.2}", avg ),
				None        => NA_STR3.to_string()
			}
		} else { NA_STR3.to_string() };
		ui.sections[ 1 ].value = fps_avr;
	}
}

//下端の情報表示を更新する(中)
fn update_footer_ui_center
(	mut q: Query<&mut Text, With<FooterUiCenter>>,
	o_falls: Option<Res<InfoNumOfFalls>>,
)
{	if let Ok( mut ui ) = q.single_mut()
 	{	let falls_count = match o_falls
		{	Some( falls ) => format!( "{:03}", falls.count ),
			None          => NA_STR3.to_string()
		};
		ui.sections[ 1 ].value = falls_count;
 	}
}

//下端の情報表示を更新する(右)
fn update_footer_ui_right
(	mut q: Query<&mut Text, With<FooterUiRight>>,
	o_bg: Option<Res<BgStars>>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	let stars_count = match o_bg
		{	Some( bg ) => format!( "{:03}", bg.stars.len() ),
			None       => NA_STR3.to_string()
		};
		ui.sections[ 1 ].value = stars_count;
	}
}

////////////////////////////////////////////////////////////////////////////////

//STARTメッセージ表示
fn show_start_message( mut q: Query<&mut Visible, With<MessageStart>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = true; }
}

//STARTメッセージ非表示
fn hide_start_message( mut q: Query<&mut Visible, With<MessageStart>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = false; }
}

//SPACEキーが入力され次第ステートを変更する
fn handle_input_space_key
(	mut inkey: ResMut<Input<KeyCode>>,
	mut event: EventWriter<GameState>,
)
{	if inkey.just_pressed( KeyCode::Space ) 
	{	event.send( GameState::Play );
		inkey.reset( KeyCode::Space ); //https://bevy-cheatbook.github.io/programming/states.html#with-input
	}
}

//End of code