use super::*;

//Pluginの手続き
pub struct PluginPlayer;
impl Plugin for PluginPlayer
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//--------------------------------------------------------------------------------
			.add_startup_system( initialize_player.system() )			// 自機の初期化
		//--------------------------------------------------------------------------------
			.add_system_set												// GameState::Play
			(	SystemSet::on_enter( GameState::Play )					// on_enter()
				.with_system( set_player_start_position.system() )		// 自機を初期値へ配置
			)
			.add_system_set												// GameState::Play
			(	SystemSet::on_update( GameState::Play )					// on_update()
				.with_system( move_sprite_player.system() )				// 自機の移動
				.with_system( handle_collision_event.system() )			// 衝突検知
				.with_system( update_life_gauge.system() )				// LIFE GAUGEを更新
			)
			.add_system_set												// GameState::Play
			(	SystemSet::on_exit( GameState::Play )					// on_exit()
				.with_system( change_player_out_of_control.system() )	// 自機を制御不能に変更
			)
		//--------------------------------------------------------------------------------
			.add_system_set												// GameState::Over
			(	SystemSet::on_update( GameState::Over )					// on_update()
				.with_system( standby_plyer_offscreen.system() )		// 画面がに出た自機を待機
				.with_system( handle_input_space_key.system() )			//
			)
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//定義と定数

//Component
struct Player;
struct LifeGauge;

//Resource
struct DamageCounter ( pub usize );
pub struct PlayTimer ( pub f64 );

//LIFE GAUGE
const GAUGE: f32 = 100.0;
const GAUGE_RATE: f32 = 18.0;
const GAUGE_PIXEL: f32 = PIXEL_PER_GRID * 0.6;
const GAUGE_POSITION: ( f32, f32 ) = ( 0.0, SCREEN_HEIGHT / 2.0 - PIXEL_PER_GRID * 0.9 );
const GAUGE_DEPTH: f32 = 20.0;

//自機のスプライト
const PLAYER_PIXEL: f32 = PIXEL_PER_GRID;
const PLAYER_COLOR: Color = Color::YELLOW;
const PLAYER_START: ( f32, f32 ) = ( 0.0, - SCREEN_HEIGHT / 4.0 );
const PLAYER_DEPTH: f32 = 10.0;

//移動可能範囲
const LEFT  : f32 = SCREEN_WIDTH  / -2.0 + PIXEL_PER_GRID;
const RIGHT : f32 = SCREEN_WIDTH  /  2.0 - PIXEL_PER_GRID;
const TOP   : f32 = SCREEN_HEIGHT /  2.0 - PIXEL_PER_GRID;
const BOTTOM: f32 = SCREEN_HEIGHT / -2.0 + PIXEL_PER_GRID;

//待機領域
const WAITING_LEFT  : f32 = LEFT   - PLAYER_PIXEL * 2.0;
const WAITING_RIGHT : f32 = RIGHT  + PLAYER_PIXEL * 2.0;
const WAITING_TOP   : f32 = TOP    + PLAYER_PIXEL * 2.0;
const WAITING_BOTTOM: f32 = BOTTOM - PLAYER_PIXEL * 2.0;

////////////////////////////////////////////////////////////////////////////////

fn initialize_player
(	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	//画面外に自機をspawnして待機させる
	let triangle = &shapes::RegularPolygon
	{	sides: 3,
		feature: shapes::RegularPolygonFeature::Radius( PLAYER_PIXEL ),
		..shapes::RegularPolygon::default()
	};
	let sprite = GeometryBuilder::build_as
	(	triangle,
		ShapeColors::new( PLAYER_COLOR ),
		bevy_prototype_lyon::utils::DrawMode::Fill( FillOptions::default() ), //名前がbevy_canvasとバッティングした
		Transform::from_translation( Vec3::new( 0.0, WAITING_BOTTOM, PLAYER_DEPTH ) )
	);
	let points = vec! //自機(三角形)の頂点情報。heronのCollisionShape用
	[	Vec3::new( 0.0, PIXEL_PER_GRID, 0.0 ),
		Vec3::new( PIXEL_PER_GRID * -0.9, PIXEL_PER_GRID * -0.5, 0.0 ),
		Vec3::new( PIXEL_PER_GRID *  0.9, PIXEL_PER_GRID * -0.5, 0.0 ),
	];
	cmds.spawn_bundle( sprite )
		.insert( Player )
		.insert( RigidBody::Sensor )
		.insert( CollisionShape::ConvexHull { points } )
		.insert( PhysicMaterial { density: 0.0, ..Default::default() } )
	;

	//正方形のスプライトを横方向へ拡大し、ライフゲージに見せる
	let ( x, y ) = GAUGE_POSITION;
	let mut sprite = SpriteBundle
	{	material : color_matl.add( Color::YELLOW.into() ),
		transform: Transform::from_translation( Vec3::new( x, y, GAUGE_DEPTH ) ),
		sprite   : Sprite::new( Vec2::new( 1.0, 1.0 ) * GAUGE_PIXEL ),
		..Default::default()
	};
	sprite.transform.apply_non_uniform_scale( Vec3::new( GAUGE_RATE, 1.0, 1.0 ) );
	cmds.spawn_bundle( sprite )
		.insert( LifeGauge );

	//Resourceを登録する
	cmds.insert_resource( DamageCounter ( 0 ) );
	cmds.insert_resource( PlayTimer ( 0.0 ) );
}

////////////////////////////////////////////////////////////////////////////////

//自機を開始位置に配置する。プレイタイマー初期化。
fn set_player_start_position
(	mut q: Query<( &mut RigidBody, &mut PhysicMaterial, &mut Transform ), With<Player>>,
	mut timer: ResMut<PlayTimer>,
	mut damage: ResMut<DamageCounter>,
)
{	//自機を画面内に移動する
	let ( mut rigid_body, mut phy_matl, mut transform ) = q.single_mut().unwrap();
	*rigid_body = RigidBody::Dynamic;
	phy_matl.density = 0.0;
	let ( x, y ) = PLAYER_START;
	transform.translation = Vec3::new( x, y, PLAYER_DEPTH );
	let angle = Quat::IDENTITY;
	transform.rotation = angle;

	//初期化
	timer.0 = 0.0;
	damage.0 = 0;
}

//キー入力に従って自機を移動する
fn move_sprite_player
(	mut q_player: Query< &mut Transform, With<Player> >,
	mut timer: ResMut<PlayTimer>,
	( time, inkey ): ( Res<Time>, Res<Input<KeyCode>> ),
)
{	//前回からの経過時間
	let time_delta = time.delta().as_secs_f32();
	timer.0 += time_delta as f64; //プレイタイマーの累積

	//キー入力を取得する
	let ( mut dx, mut dy ) = ( 0.0, 0.0 );
	if inkey.pressed( KeyCode::Left  ) { dx += -PLAYER_PIXEL }
	if inkey.pressed( KeyCode::Right ) { dx +=  PLAYER_PIXEL }
	if inkey.pressed( KeyCode::Up    ) { dy +=  PLAYER_PIXEL }
	if inkey.pressed( KeyCode::Down  ) { dy += -PLAYER_PIXEL }

	//スプライトの表示位置を更新する
	let mut transform = q_player.single_mut().unwrap();
	let position = &mut transform.translation;
	let mut new_x = position.x + dx * time_delta * 10.0;
	let mut new_y = position.y + dy * time_delta * 10.0;
	new_x = if new_x < LEFT   { LEFT   } else { new_x };
	new_x = if new_x > RIGHT  { RIGHT  } else { new_x };
	new_y = if new_y > TOP    { TOP    } else { new_y };
	new_y = if new_y < BOTTOM { BOTTOM } else { new_y };
	position.x = new_x;
	position.y = new_y;
}

//衝突検知
fn handle_collision_event
(	q_player: Query< Entity, With<Player> >,
	mut events: EventReader<CollisionEvent>,
	mut damage: ResMut<DamageCounter>,
)
{	let player = q_player.single().unwrap();

	events.iter().for_each( | event |
	{	if let CollisionEvent::Started( id1, id2 ) = event
		{	if id1.rigid_body_entity() == player
			|| id2.rigid_body_entity() == player { damage.0 += 1 }
		}
	} );
}

//ライフゲージを更新する。ライフがゼロならGameOverイベントを送信する
fn update_life_gauge
(	mut q_gauge: Query<( &mut Transform, &Handle<ColorMaterial> ), With<LifeGauge>>,
	mut assets_color_matl: ResMut<Assets<ColorMaterial>>,
	damage: Res<DamageCounter>,
	mut event: EventWriter<GameState>,
)
{	let ( mut transform, handle ) = q_gauge.single_mut().unwrap();
	let life = GAUGE - damage.0 as f32; //残りHP

	//LIFE GAUGEの横方向の拡大率を変える(長方形の長さが変わる)
	let scale = &mut transform.scale;
	if 	scale[ 0 ] > 0.0
	{	scale[ 0 ] = ( GAUGE_RATE / GAUGE ) * life;
		if scale[ 0 ] < 0.0 { scale[ 0 ] = 0.0 }
	}

	//色を変える(黄色⇒赤色)
	let color_matl = assets_color_matl.get_mut( handle ).unwrap();
	let ( r, g, b ) = ( 1.0, life / 100.0, 0.0 );
	color_matl.color = Color::rgb( r, g, b );

	//LIFEが0ならゲームオーバー
	if life <= 0.0 { event.send( GameState::Over ); }
}

//ライフがゼロになった自機はコントロール不能
fn change_player_out_of_control
(	mut q_player: Query< &mut PhysicMaterial, With<Player> >,
	mut q_ui : Query<&mut Visible, With<MessageOver>>,
)
{	//自機に密度を与えて、以降の運動をheronに任せる
	let mut phy_matl = q_player.single_mut().unwrap();
	phy_matl.density = 1.0;

	//GAME OVERメッセージ表示
	if let Ok( mut ui ) = q_ui.single_mut() { ui.is_visible = true }
}

//画面外に出た自機を待機させる
fn standby_plyer_offscreen( mut q: Query<( &mut RigidBody, &Transform ), With<Player> > )
{	if let Ok ( ( mut rigid_body, transform ) ) = q.single_mut()
	{	if *rigid_body == RigidBody::Dynamic
		{	if ! ( WAITING_LEFT..=WAITING_RIGHT ).contains( &transform.translation.x )
			|| ! ( WAITING_BOTTOM..=WAITING_TOP ).contains( &transform.translation.y )
			{	*rigid_body = RigidBody::Sensor
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////

//SPACEキーが入力され次第ステートを変更する
fn handle_input_space_key
(	mut q_ui : Query<&mut Visible, With<MessageOver>>,
	mut q_gauge: Query<( &mut Transform, &Handle<ColorMaterial> ), With<LifeGauge>>,
	mut assets_color_matl: ResMut<Assets<ColorMaterial>>,
	mut inkey: ResMut<Input<KeyCode>>,
	mut event: EventWriter<GameState>,
)
{	if inkey.just_pressed( KeyCode::Space ) 
	{	//GAME OVERメッセージ非表示
		if let Ok( mut ui ) = q_ui.single_mut() { ui.is_visible = false }

		//LIFE GAUGEを初期化する
		let ( mut transform, handle ) = q_gauge.single_mut().unwrap();
		let scale = &mut transform.scale;
		scale[ 0 ] = GAUGE_RATE;
		let color_matl = assets_color_matl.get_mut( handle ).unwrap();
		color_matl.color = Color::YELLOW;

		//replay
		event.send( GameState::Play );
		inkey.reset( KeyCode::Space ); //https://bevy-cheatbook.github.io/programming/states.html#with-input
	}
}

//End of code