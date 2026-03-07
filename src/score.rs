use bevy::prelude::*;

use crate::startup::*;
use crate::ball::*;
use crate::paddles::*;
use crate::collision::*;

#[derive(Resource)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

#[derive(Event)]
pub struct Scored {
    scorer: Entity,
}


#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

// Scoreboard UI
// -----------------------------------------------------------------
pub fn spawn_scoreboard(mut commands: Commands) {
    let container = Node {
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    let header = Node {
        width: px(200.0),
        height: px(100.0),
        ..default()
    };

    let player_score = (
        PlayerScore,
        Text::new("0"),
        TextFont::from_font_size(72.0),
        TextColor(Color::srgb(255., 255., 0.)),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.0),
            left: px(25.0),
            ..default()
        },
    );

    let ai_score = (
        AiScore,
        Text::new("0"),
        TextFont::from_font_size(90.0),
        TextColor(Color::srgb(255., 0., 255.)),
        TextLayout::new_with_justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.0),
            left: px(25.0),
            ..default()
        },
    );

    commands.spawn((
            container,
            children![(header, children![player_score, ai_score])],
    ));
}
// -----------------------------------------------------------------
// End Scoreboard UI

// Scoreboard Detection {{{
// -----------------------------------------------------------------
pub fn detect_goal(
    ball: Single<(&Position, &Collider), With<Ball>>,
    player: Single<Entity, (With<Player>, Without<Ai>)>,
    ai: Single<Entity, (With<Ai>, Without<Player>)>,
    window: Single<&Window>,
    mut commands: Commands
) {
    let (ball_position, ball_collider) = ball.into_inner();
    let half_window_size = window.resolution.size() / 2.0;
 
    if ball_position.0.x - ball_collider.half_size().x > half_window_size.x {
        commands.trigger(Scored { scorer: *player })
    }

    if ball_position.0.x + ball_collider.half_size().x < -half_window_size.x {
        commands.trigger(Scored { scorer: *ai });
    }
}

pub fn reset_ball(
    _event: On<Scored>,
    ball: Single<(&mut Position, &mut Transform), With<Ball>>,
) {
    let (mut ball_position, mut ball_transform) = ball.into_inner();

    ball_position.0 = Vec2::ZERO;
    ball_transform.translation = Vec2::new(BALL_SPEED, 0.0).extend(0.0);
}

pub fn update_score(
    event: On<Scored>,
    mut score: ResMut<Score>,
    is_ai: Query<&Ai>,
    is_player: Query<&Player>
) {
    if is_ai.get(event.scorer).is_ok() {
        score.ai += 1;
        info!("Ai Scored! {} - {}", score.player, score.ai);
    }

    if is_player.get(event.scorer).is_ok() {
        score.player += 1;
        info!("Player Scored! {} - {}", score.player, score.ai);
    }
}
// -----------------------------------------------------------------
// End Scoreboard Detection }}}


