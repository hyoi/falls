use super::*;

//Pluginの手続き
pub struct PluginFalls;
impl Plugin for PluginFalls
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//--------------------------------------------------------------------------------
			.add_startup_system( initialize_falls.system() )		// 落下物の初期化
		//--------------------------------------------------------------------------------
			.add_system_set											// GameState::Start
			(	SystemSet::on_update( GameState::Start )			// on_update()
				.with_system( falling_meteors_onscreen.system() )	// 落下物を投入
				.with_system( standby_meteors_offscreen.system() )	// 落下物を待機
			)
		//--------------------------------------------------------------------------------
			//Play中のPause処理のため、GameState::Playを独立させる。
			.add_system_set											// GameState::Play
			(	SystemSet::on_update( GameState::Play )				// on_update()
				.with_system( falling_meteors_onscreen.system() )	// 落下物を投入
				.with_system( standby_meteors_offscreen.system() )	// 落下物を待機
			)
		//--------------------------------------------------------------------------------
			.add_system_set											// GameState::Over
			(	SystemSet::on_update( GameState::Over )				// on_update()
				.with_system( falling_meteors_onscreen.system() )	// 落下物を投入
				.with_system( standby_meteors_offscreen.system() )	// 落下物を待機
			)
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//定義と定数

//Component
struct Meteor;

//Resource
struct FallingRhythm { timer: Timer }					 //落下物の発生タイマー
pub struct InfoNumOfFalls { pub count: usize }			 //落下中の数

//落下物
const SPRITE_PNG_FILE: &str = "sprites/meteor.png";		 //画像ファイル
const METEOR_SPAWN_WAIT: f32 = 0.0166;					 //発生タイマーのウエイト
const MAX_NUM_OF_FALLS: usize = 130;					 //最大数
const SPACE_GRAVITY: [ f32; 2 ] = [ 0.0, -9.81 * 10.0 ]; //宇宙重力

//移動可能範囲
const LEFT  : f32 = SCREEN_WIDTH  / -2.0 - PIXEL_PER_GRID;
const RIGHT : f32 = SCREEN_WIDTH  /  2.0 + PIXEL_PER_GRID;
const TOP   : f32 = SCREEN_HEIGHT /  2.0 + PIXEL_PER_GRID;
const BOTTOM: f32 = SCREEN_HEIGHT / -2.0 - PIXEL_PER_GRID;

////////////////////////////////////////////////////////////////////////////////

//落下物の位置と速度を乱数で決める
fn generate_position_and_velocity() -> ( Vec3, Vec2 )
{	let mut rng = rand::thread_rng();
	let p = Vec3::new( rng.gen_range( LEFT..=RIGHT ), TOP, 0.0 );
	let v = Vec2::new( rng.gen_range( -0.5..= 0.5 ), rng.gen_range( -20.0..=-5.0 ) ) * PIXEL_PER_GRID;
	( p, v )
}

//落下物の初期化
fn initialize_falls
(	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
	asset_svr: Res<AssetServer>,
)
{	//画面外に落下物をspawnして待機させる
	( 0..MAX_NUM_OF_FALLS ).for_each( |_|
	{	let ( p, v ) = generate_position_and_velocity();
		let sprite = SpriteBundle
		{	sprite   : Sprite::new( Vec2::new( 1.0, 1.0 ) * PIXEL_PER_GRID ),
			material : color_matl.add( asset_svr.load( SPRITE_PNG_FILE ).into() ),
			transform: Transform::from_translation( p ),
			..Default::default()
		};
		cmds.spawn_bundle( sprite )
			.insert( Meteor )
			.insert( RigidBody::Sensor )
			.insert( CollisionShape::Sphere { radius: PIXEL_PER_GRID / 2.0 } )
			.insert( Velocity::from( v ) )
			.insert( PhysicMaterial { restitution: 0.2, ..Default::default() } )
			.insert( RotationConstraints::lock() )
		;
	} );

	//Resourceを登録する
	cmds.insert_resource( FallingRhythm { timer: Timer::from_seconds( METEOR_SPAWN_WAIT, false ) } );
	cmds.insert_resource( Gravity::from( Vec2::from_slice_unaligned( &SPACE_GRAVITY ) ) );
	cmds.insert_resource( InfoNumOfFalls{ count: 0 } );
}

//待機中だった落下物を画面上端に投入する
fn falling_meteors_onscreen
(	q: Query<( &mut RigidBody, &mut Transform, &mut Velocity ), With<Meteor>>,
	( mut falling, time ): ( ResMut<FallingRhythm>, Res<Time> ),
	mut info: ResMut<InfoNumOfFalls>,
)
{	//落下物の発生をタイマーで調節
	if ! falling.timer.tick( time.delta() ).finished() { return }
	falling.timer.reset();

	//落下物を一つ投入する
	let mut flag = 1;
	let mut count = 0;
	q.for_each_mut( | ( mut rigid_body, mut transform, mut velocity ) |
	{	if flag == 1 && *rigid_body == RigidBody::Sensor
		{	flag = 0;
			*rigid_body = RigidBody::Dynamic;
			let ( p, v ) = generate_position_and_velocity();
			transform.translation = p;
			velocity.linear = v.extend( 0.0 );
		}
		if *rigid_body == RigidBody::Dynamic { count += 1 }
	} );
	info.count = count;
}

//画面外に出た落下物を待機させる
fn standby_meteors_offscreen( q: Query<( &mut RigidBody, &Transform ), With<Meteor>> )
{	q.for_each_mut( | ( mut rigid_body, transform ) |
	{	if *rigid_body == RigidBody::Dynamic
		{	if ! ( LEFT..=RIGHT ).contains( &transform.translation.x )
			|| ! ( BOTTOM..=TOP ).contains( &transform.translation.y )
			{	*rigid_body = RigidBody::Sensor
			}	
		}
	} );
}

//End of code