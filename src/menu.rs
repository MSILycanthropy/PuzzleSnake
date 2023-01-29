use bevy::prelude::*;

use crate::{despawn, GameState};

const MENU_FONT: &str = "fonts/impact.ttf";

// TODO: More menus :)
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Disabled,
    Main,
}

#[derive(Component)]
enum MenuAction {
    Play,
    Exit,
}

#[derive(Component)]
struct OnMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Main)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup_system))
            .add_system_set(
                SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup_system),
            )
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_action_system))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawn::<OnMenu>));
    }
}

fn menu_setup_system(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

fn main_menu_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            ..default()
                        },
                        ..default()
                    },
                    MenuAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load(MENU_FONT),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(200.), Val::Px(50.)),
                            margin: UiRect {
                                left: Val::Px(0.),
                                right: Val::Px(0.),
                                top: Val::Px(10.),
                                bottom: Val::Px(0.),
                            },
                            ..default()
                        },
                        ..default()
                    },
                    MenuAction::Exit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load(MENU_FONT),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}

fn menu_action_system(
    query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                MenuAction::Play => {
                    let _ = menu_state.set(MenuState::Disabled);
                    let _ = game_state.set(GameState::Playing);
                }
                MenuAction::Exit => app_exit_events.send(bevy::app::AppExit),
            }
        }
    }
}
