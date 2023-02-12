use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn, GameState, UiAssets};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Disabled,
    Main,
}

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
struct OnMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(MenuState::Disabled)
            .add_enter_system(GameState::Menu, menu_setup_system)
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

fn menu_setup_system(mut commands: Commands) {
    commands.insert_resource(NextState(MenuState::Main));
}

fn main_menu_setup_system(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
            parent.spawn(ImageBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(480.0), Val::Px(160.0)),
                    ..default()
                },
                image: ui_assets.logo.clone().into(),
                ..default()
            });

            parent.spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                        margin: UiRect {
                            top: Val::Px(60.0),
                            ..default()
                        },
                        ..default()
                    },
                    image: ui_assets.start_button.clone().into(),
                    ..default()
                },
                PlayButton,
            ));

            #[cfg(not(target_arch = "wasm32"))]
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.), Val::Px(80.)),
                        margin: UiRect {
                            top: Val::Px(20.),
                            ..default()
                        },
                        ..default()
                    },
                    image: ui_assets.exit_button.clone().into(),
                    ..default()
                },
                ExitButton,
            ));
        });
}

pub fn button_interacted<T: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<T>)>,
) -> bool {
    query
        .iter()
        .any(|interaction| *interaction == Interaction::Clicked)
}

pub fn button_play(mut commands: Commands) {
    commands.insert_resource(NextState(MenuState::Disabled));
    commands.insert_resource(NextState(GameState::Playing));
}

pub fn button_exit(mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    app_exit_events.send(bevy::app::AppExit);
}
