use super::font::fonts;

use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;
use std::time::Duration;
// use bevy::app::prelude::*;
// use bevy::core::{Time, Timer};
// use bevy::ecs::{IntoQuerySystem, Res, ResMut};

/// An App Plugin that displays diagnostics
pub struct DisplayDiagnosticsPlugin {
    pub debug: bool,
    pub wait_duration: Duration,
    pub filter: Option<Vec<DiagnosticId>>,
}

/// State used by the [DisplayDiagnosticsPlugin]
pub struct DisplayDiagnosticsState {
    timer: Timer,
    filter: Option<Vec<DiagnosticId>>,
}

pub struct DisplayText(bool, String);

impl Default for DisplayDiagnosticsPlugin {
    fn default() -> Self {
        DisplayDiagnosticsPlugin {
            debug: false,
            wait_duration: Duration::from_secs(1),
            filter: None,
        }
    }
}

impl Plugin for DisplayDiagnosticsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(DisplayDiagnosticsState {
            timer: Timer::new(self.wait_duration, true),
            filter: self.filter.clone(),
        });
        // app.init_resource::<FontMap>();
        app.add_startup_system(Self::setup.system());
        app.add_system(Self::display_diagnostics_text_system.system());
        if self.debug {
            app.add_system_to_stage(
                stage::POST_UPDATE,
                Self::display_diagnostics_debug_system.system(),
            );
        } else {
            app.add_system_to_stage(
                stage::POST_UPDATE,
                Self::store_diagnostics_text_system.system(),
            );
        }
    }
}

impl DisplayDiagnosticsPlugin {
    pub fn filtered(filter: Vec<DiagnosticId>) -> Self {
        DisplayDiagnosticsPlugin {
            filter: Some(filter),
            ..Default::default()
        }
    }

    fn display_diagnostic(buffer: &mut String, diagnostic: &Diagnostic) {
        if let Some(value) = diagnostic.value() {
            *buffer += &*format!("{:<12}: {:<10.6}", diagnostic.name, value);
            if let Some(average) = diagnostic.average() {
                *buffer += &*format!(" (avg {:.6})", average);
            }
            *buffer += "\n";
        }
    }

    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        // mut font_map: ResMut<FontMap>,
    ) {
        commands
            .spawn(TextComponents {
                text: Text {
                    font: asset_server.load(fonts::JETBRAINSMONO_LIGHT),
                    value: "Waiting...".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        font_size: 10.0,
                    },
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        right: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(DisplayText(true, "".to_string()));
    }

    pub fn store_diagnostics_text_system(
        mut state: ResMut<DisplayDiagnosticsState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
        mut text_query: Query<&mut DisplayText>,
    ) {
        state.timer.tick(time.delta_seconds);
        if state.timer.finished {
            // if let Some((&mut text, display_text)) = text_query.iter().iter().next() {
            for mut display_text in &mut text_query.iter() {
                if display_text.0 {
                    let mut buffer = String::new();
                    if let Some(ref filter) = state.filter {
                        for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                            Self::display_diagnostic(&mut buffer, diagnostic);
                        }
                    } else {
                        for diagnostic in diagnostics.iter() {
                            Self::display_diagnostic(&mut buffer, diagnostic);
                        }
                    }
                    display_text.1 = buffer;
                } else {
                    display_text.1 = "".to_string();
                }
            }
        }
    }

    fn display_diagnostics_text_system(mut text_query: Query<(&mut Text, &DisplayText)>) {
        for (mut text, display_text) in &mut text_query.iter() {
            if text.value != display_text.1 {
                text.value = display_text.1.clone();
            }
        }
    }

    pub fn display_diagnostics_debug_system(
        mut state: ResMut<DisplayDiagnosticsState>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
    ) {
        state.timer.tick(time.delta_seconds);
        if state.timer.finished {
            println!("Diagnostics (Debug):");
            println!("{}", "-".repeat(93));
            if let Some(ref filter) = state.filter {
                for diagnostic in filter.iter().map(|id| diagnostics.get(*id).unwrap()) {
                    println!("{:#?}\n", diagnostic);
                }
            } else {
                for diagnostic in diagnostics.iter() {
                    println!("{:#?}\n", diagnostic);
                }
            }
        }
    }
}
