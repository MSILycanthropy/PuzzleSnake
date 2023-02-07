use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn, GameState};

const MENU_FONT: &str = "fonts/impact.ttf";

// TODO: More menus :)
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Disabled,
    Main,
}

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct OnMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(MenuState::Main)
            .add_enter_system(MenuState::Main, main_menu_setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(MenuState::Main)
                    .with_system(button_play.run_if(button_interacted::<PlayButton>))
                    .with_system(button_exit.run_if(button_interacted::<ExitButton>))
                    .into(),
            )
            .add_exit_system(MenuState::Main, despawn::<OnMenu>);
    }
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
                    PlayButton,
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
                    ExitButton,
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

fn button_interacted<T: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<T>)>,
) -> bool {
    query
        .iter()
        .any(|interaction| *interaction == Interaction::Clicked)
}

fn button_play(mut commands: Commands) {
    commands.insert_resource(NextState(MenuState::Disabled));
    commands.insert_resource(NextState(GameState::Playing));
}

fn button_exit(mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    app_exit_events.send(bevy::app::AppExit);
}
