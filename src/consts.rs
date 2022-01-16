use super::*;

//アプリのTitle
pub const APP_TITLE: &str = "falls";

//マップの縦横のグリッド数（今回はあまり意味なし）
pub const MAP_WIDTH : usize = 25;
pub const MAP_HEIGHT: usize = 40;

//表示倍率、ウィンドウの縦横pixel数と背景色
pub const SCREEN_SCALING: usize = 3;
pub const PIXEL_PER_GRID: f32   = ( 8 * SCREEN_SCALING ) as f32;
pub const SCREEN_WIDTH  : f32   = PIXEL_PER_GRID * MAP_WIDTH  as f32;
pub const SCREEN_HEIGHT : f32   = PIXEL_PER_GRID * MAP_HEIGHT as f32;
pub const SCREEN_BGCOLOR: Color = Color::BLACK;

////////////////////////////////////////////////////////////////////////////////

pub const FONT_FILE: &str = "fonts/ReggaeOne-Regular.ttf";

pub const NA_STR3: &str = "---";
pub const NA_TIME: &str = "-.--";

pub type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

#[derive(Component)]
pub struct MessagePause;
pub const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_FILE, PIXEL_PER_GRID * 5.0, Color::SILVER ),
];

#[derive(Component)]
pub struct MessageStart;
pub const MESSAGE_START: [ MessageSect; 2 ] =
[	( "GAME START", FONT_FILE, PIXEL_PER_GRID * 5.0, Color::SILVER ),
	( "\nHit [SPACE] Key", FONT_FILE, PIXEL_PER_GRID * 2.0, Color::WHITE ),
];

#[derive(Component)]
pub struct MessageOver;
pub const MESSAGE_OVER: [ MessageSect; 2 ] =
[	( "GAME OVER", FONT_FILE, PIXEL_PER_GRID * 5.0, Color::SILVER ),
	( "\nReplay?\nHit [SPACE] Key", FONT_FILE, PIXEL_PER_GRID * 2.0, Color::WHITE ),
];

#[derive(Component)]
pub struct HeaderUiLeft;
pub const HEADER_UI_LEFT: [ MessageSect; 3 ] =
[	( "バリヤー ", FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
	( NA_STR3   , FONT_FILE, PIXEL_PER_GRID * 1.5, Color::WHITE ),
	( "%"	    , FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
];

#[derive(Component)]
pub struct HeaderUiRight;
pub const HEADER_UI_RIGHT: [ MessageSect; 3 ] =
[	( "生存 ", FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
	( NA_TIME, FONT_FILE, PIXEL_PER_GRID * 1.5, Color::WHITE  ),
	( "秒"	 , FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
];

#[derive(Component)]
pub struct FooterUiLeft;
pub const FOOTER_UI_LEFT: [ MessageSect; 2 ] =
[	( "FPS " , FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
	( NA_STR3, FONT_FILE, PIXEL_PER_GRID * 1.5, Color::WHITE  ),
];

#[derive(Component)]
pub struct FooterUiCenter;
pub const FOOTER_UI_CENTER: [ MessageSect; 3 ] =
[	( "落下物 ", FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
	( NA_STR3  , FONT_FILE, PIXEL_PER_GRID * 1.5, Color::WHITE  ),
	( "個"	   , FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
];

#[derive(Component)]
pub struct FooterUiRight;
pub const FOOTER_UI_RIGHT: [ MessageSect; 3 ] =
[	( "背景の星 ", FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
	( NA_STR3	, FONT_FILE, PIXEL_PER_GRID * 1.5, Color::WHITE  ),
	( "個"		, FONT_FILE, PIXEL_PER_GRID * 1.2, Color::ORANGE ),
];

//End of code.