use super::*;

//Pluginの手続き
pub struct PluginBgStars;
impl Plugin for PluginBgStars
{	fn build( &self, app: &mut App )
	{	app
		//--------------------------------------------------------------------------------
		.add_startup_system( initialize_bg_stars )	// 背景の星の初期状態を決める
		.add_system( scroll_bg_stars )				// 背景の星をスクロール
		.add_system( twinkle_bg_stars )				// 背景の星を瞬かせる
		//--------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////

//描画範囲
const LEFT  : f32 = SCREEN_WIDTH  / -2.0;
const RIGHT : f32 = SCREEN_WIDTH  /  2.0;
const TOP   : f32 = SCREEN_HEIGHT /  2.0;
const BOTTOM: f32 = SCREEN_HEIGHT / -2.0;

#[derive(Component)]
struct BgStarV ( f32, f32 );

#[derive(Component)]
struct BgStarHue ( f32 );

struct BgTimer ( Timer );

////////////////////////////////////////////////////////////////////////////////

fn initialize_bg_stars( mut cmds: Commands )
{	let mut rng = rand::thread_rng();
	let ( saturation, lightness, alpha ) = ( 1.0, 0.6, 1.0 );

	( 0..300 ).for_each( |_|
	{	let ( x, y )   = ( rng.gen_range( LEFT..RIGHT ), rng.gen_range( BOTTOM..TOP ) );
		let radius     = rng.gen_range( 0.2..=1.5 );
		let ( vx, vy ) = ( 0.0, if radius > 1.0 { -150.0 } else if radius > 0.5 { -100.0 } else { -50.0 } );
		let hue        = rng.gen_range( 0.0..360.0 );
		let color      = Color::Hsla { hue, saturation, lightness, alpha };

		cmds.spawn_bundle( SpriteBundle::default() )
			.insert( Sprite
			{	color,
				custom_size: Some( Vec2::new( radius, radius ) ),
				..default()
			} )
			.insert( Transform::from_translation( Vec3::new( x, y, 0.0 ) ) )
			.insert( BgStarV ( vx, vy ) )
			.insert( BgStarHue ( hue ) );
	} );
	
	let timer = Timer::from_seconds( 0.02, false );

	cmds.insert_resource( BgTimer ( timer ) );
}

fn scroll_bg_stars
(	mut q_star: Query<(&BgStarV, &mut Transform)>,
	time: Res<Time>,
)
{	//前回からの経過時間
	let time_delta = time.delta().as_secs_f32();

	q_star.for_each_mut
	(	| ( star, mut transform ) |
		{	let BgStarV ( vx, vy ) = star;

			let position = &mut transform.translation;
			position.x += vx * time_delta;
			position.y += vy * time_delta;

			if position.y < BOTTOM { position.y += SCREEN_HEIGHT }
		}
	);
}

fn twinkle_bg_stars
(	mut q_star: Query<(&mut BgStarHue, &mut Sprite)>,
	time: Res<Time>,
)
{	let time_delta = time.delta().as_secs_f32();
	let ( saturation, lightness, alpha ) = ( 1.0, 0.6, 1.0 );

	q_star.for_each_mut
	(	| ( mut star, mut sprite ) |
		{	let BgStarHue ( hue ) = *star;
			let mut hue = hue + 360.0 * time_delta;
			if hue > 360.0 { hue %= 360.0 }
			*star = BgStarHue ( hue );

			sprite.color = Color::Hsla { hue, saturation, lightness, alpha };
		}
	);
}

//End of code