use super::*;

//Pluginの手続き
pub struct PluginMeteor;
impl Plugin for PluginMeteor
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//--------------------------------------------------------------------------------
			.insert_resource( MeteorTimer ( Timer::from_seconds( 0.02, false ) ) )
			.insert_resource( BitNums ( 0 ) )
			.insert_resource( Gravity::from( Vec2::new( 0.0, -9.81 * 10.0 ) ) )
		//--------------------------------------------------------------------------------
			.add_system_set											// GameState::Start
			(	SystemSet::on_update( GameState::Start )			// on_update()
				.with_system( spawn_sprite_meteor.system() )		// 落下物の追加
				.with_system( despawn_sprite_meteor.system() )		// 落下物の削除
			)
		//--------------------------------------------------------------------------------
			//Play中のPause処理を実現するため落下物処理は一つにまとめられない。
			.add_system_set											// GameState::Play
			(	SystemSet::on_update( GameState::Play )				// on_update()
				.with_system( spawn_sprite_meteor.system() )		// 落下物の追加
				.with_system( despawn_sprite_meteor.system() )		// 落下物の削除
			)
		//--------------------------------------------------------------------------------
			.add_system_set											// GameState::Over
			(	SystemSet::on_update( GameState::Over )				// on_update()
				.with_system( spawn_sprite_meteor.system() )		// 落下物の追加
				.with_system( despawn_sprite_meteor.system() )		// 落下物の削除
			)
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//定義と定数

//Component
struct Meteor {	bit_no: u128,}

//Resource
struct MeteorTimer ( Timer );		//落下物の発生タイマー
pub struct BitNums ( pub u128, );	//落下物発生数の管理用(bitで管理。最大128個)
impl BitNums
{	//ビットがゼロの位置を返す(1～128)。見つからない場合は0。
	fn search_zero_bit( &self ) -> u128
	{	let mut bit_no = 0;
		for i in 0..128
		{	let bit = 1 << i;
			if self.0 & bit == 0 { bit_no = bit; break }
		}

		bit_no
	}
}

//落下物
const SPRITE_PNG_FILE: &str = "sprites/meteor.png";

//移動可能範囲
const LEFT  : f32 = SCREEN_WIDTH  / -2.0 - PIXEL_PER_GRID;
const RIGHT : f32 = SCREEN_WIDTH  /  2.0 + PIXEL_PER_GRID;
const TOP   : f32 = SCREEN_HEIGHT /  2.0 + PIXEL_PER_GRID;
const BOTTOM: f32 = SCREEN_HEIGHT / -2.0 - PIXEL_PER_GRID;

////////////////////////////////////////////////////////////////////////////////

//落下物を追加する
fn spawn_sprite_meteor
(	mut bits: ResMut<BitNums>,
	mut timer: ResMut<MeteorTimer>,
	mut cmds: Commands,
	time: Res<Time>,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
	asset_svr: Res<AssetServer>,
)
{	//落下物の発生頻度をタイマーで調節
	if ! timer.0.tick( time.delta() ).finished() { return }
	timer.0.reset();

	//落下物に割り当てるIDを探す
	let bit_no = bits.search_zero_bit();
	if bit_no == 0  { return }	//IDに空きが見つからないなら関数脱出
	bits.0 |= bit_no;

//	println!( "{:03} {:>0128b}", bits.0.count_ones(), bits.0 );	// for Debug

	//落下物を配置する
	let mut rng = rand::thread_rng();
	let position = Vec3::new( rng.gen_range( LEFT..=RIGHT ), TOP, 0.0 );
	let square = Vec2::new( PIXEL_PER_GRID, PIXEL_PER_GRID );
	let sprite = SpriteBundle
	{	sprite   : Sprite::new( square ),
		material : color_matl.add( asset_svr.load( SPRITE_PNG_FILE ).into() ),
		transform: Transform::from_translation( position ),
		..Default::default()
	};
	let v = Vec2::new
	(	PIXEL_PER_GRID * rng.gen_range(  -0.5..= 0.5 ),
		PIXEL_PER_GRID * rng.gen_range( -20.0..=-5.0 )
	);

	//RigidBodyをセットすることで、落下＆衝突の動作はheron任せにする
	cmds
		.spawn_bundle( sprite )
		.insert( Meteor { bit_no } )
		.insert( RigidBody::Dynamic )
		.insert( CollisionShape::Sphere { radius: PIXEL_PER_GRID / 2.0 } )
		.insert( Velocity::from( v ).with_angular( AxisAngle::new( Vec3::Z, 0.0 ) ) )
		.insert( PhysicMaterial { restitution: 0.2, ..Default::default() } )
		.insert( RotationConstraints::lock() )
	;
}

//落下物を削除する
fn despawn_sprite_meteor
(	q_meteor: Query<( &Meteor, &Transform, Entity )>,
	mut bits: ResMut<BitNums>,
	mut cmds: Commands,
)
{	//表示領域を出た隕石を削除する
	q_meteor.for_each( | ( meteor, transform, id ) |
	{	let x = transform.translation.x;
		let y = transform.translation.y;

		if ! ( LEFT..=RIGHT ).contains( &x ) || ! ( BOTTOM..=TOP ).contains( &y )
		{	bits.0 &= ! meteor.bit_no;
			cmds.entity( id ).despawn();
		}
	} );
}

//End of code