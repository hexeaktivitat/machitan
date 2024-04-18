use bevy::prelude::*;

use crate::ApplicationState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, menu_setup.in_set(UiSet))
            .add_systems(Update, (clear_menu, main_menu).in_set(UiSet));
    }
}

fn menu_setup(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::SEA_GREEN),
                    background_color: BackgroundColor(Color::ALICE_BLUE),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            // font: server.load("fonts/TitilliumWeb-SemiBold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.1, 0.1, 0.1),
                            ..default()
                        },
                    ));
                });
        });
}

fn main_menu(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    state: Res<State<ApplicationState>>,
    mut next_state: ResMut<NextState<ApplicationState>>,
) {
    for (interaction, mut color, mut border, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => next_state.set(ApplicationState::InGame),
            Interaction::Hovered => text.sections[0].value = "GANBARUZO!".into(),
            Interaction::None => text.sections[0].value = "Start Game".into(),
        }
    }
}

fn clear_menu(
    mut commands: Commands,
    mut query: Query<Entity, With<Node>>,
    state: Res<State<ApplicationState>>,
) {
    if state.get() != &ApplicationState::Menu {
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}
