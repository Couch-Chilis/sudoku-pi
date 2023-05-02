mod button_bundle;
mod flex_bundles;
mod interaction;
mod layout;
mod toggle_bundle;

use bevy::prelude::{App, Plugin};
pub use button_bundle::*;
pub use flex_bundles::*;
pub use interaction::*;
pub use toggle_bundle::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            layout::layout_system,
            interaction::mouse_interaction,
            interaction::button_interaction,
        ));
    }
}
