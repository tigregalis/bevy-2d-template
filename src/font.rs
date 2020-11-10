use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Default)]
pub struct FontMap {
    fonts: HashMap<&'static str, Handle<Font>>,
}

impl FontMap {
    pub fn get_or_load(
        &mut self,
        asset_server: &Res<AssetServer>,
        path: &'static str,
    ) -> Handle<Font> {
        let font = self.fonts.entry(path).or_insert(asset_server.load(path));
        font.clone()
    }
    #[allow(dead_code)]
    pub fn unload(&mut self, path: &'static str) {
        self.fonts.remove(path);
    }
}

#[allow(dead_code)]
pub mod fonts {
    pub const FIRASANS_BLACK: &str = "fonts/FiraSans-Black.ttf";
    pub const FIRASANS_BLACKITALIC: &str = "fonts/FiraSans-BlackItalic.ttf";
    pub const FIRASANS_BOLD: &str = "fonts/FiraSans-Bold.ttf";
    pub const FIRASANS_BOLDITALIC: &str = "fonts/FiraSans-BoldItalic.ttf";
    pub const FIRASANS_EXTRABOLD: &str = "fonts/FiraSans-ExtraBold.ttf";
    pub const FIRASANS_EXTRABOLDITALIC: &str = "fonts/FiraSans-ExtraBoldItalic.ttf";
    pub const FIRASANS_EXTRALIGHT: &str = "fonts/FiraSans-ExtraLight.ttf";
    pub const FIRASANS_EXTRALIGHTITALIC: &str = "fonts/FiraSans-ExtraLightItalic.ttf";
    pub const FIRASANS_ITALIC: &str = "fonts/FiraSans-Italic.ttf";
    pub const FIRASANS_LIGHT: &str = "fonts/FiraSans-Light.ttf";
    pub const FIRASANS_LIGHTITALIC: &str = "fonts/FiraSans-LightItalic.ttf";
    pub const FIRASANS_MEDIUM: &str = "fonts/FiraSans-Medium.ttf";
    pub const FIRASANS_MEDIUMITALIC: &str = "fonts/FiraSans-MediumItalic.ttf";
    pub const FIRASANS_REGULAR: &str = "fonts/FiraSans-Regular.ttf";
    pub const FIRASANS_SEMIBOLD: &str = "fonts/FiraSans-SemiBold.ttf";
    pub const FIRASANS_SEMIBOLDITALIC: &str = "fonts/FiraSans-SemiBoldItalic.ttf";
    pub const FIRASANS_THIN: &str = "fonts/FiraSans-Thin.ttf";
    pub const FIRASANS_THINITALIC: &str = "fonts/FiraSans-ThinItalic.ttf";
    pub const JETBRAINSMONO_BOLDITALIC: &str = "fonts/JetBrainsMono-Bold-Italic.ttf";
    pub const JETBRAINSMONO_BOLD: &str = "fonts/JetBrainsMono-Bold.ttf";
    pub const JETBRAINSMONO_EXTRABOLDITALIC: &str = "fonts/JetBrainsMono-ExtraBold-Italic.ttf";
    pub const JETBRAINSMONO_EXTRABOLD: &str = "fonts/JetBrainsMono-ExtraBold.ttf";
    pub const JETBRAINSMONO_EXTRALIGHTITALIC: &str = "fonts/JetBrainsMono-ExtraLight-Italic.ttf";
    pub const JETBRAINSMONO_EXTRALIGHT: &str = "fonts/JetBrainsMono-ExtraLight.ttf";
    pub const JETBRAINSMONO_ITALIC: &str = "fonts/JetBrainsMono-Italic.ttf";
    pub const JETBRAINSMONO_LIGHTITALIC: &str = "fonts/JetBrainsMono-Light-Italic.ttf";
    pub const JETBRAINSMONO_LIGHT: &str = "fonts/JetBrainsMono-Light.ttf";
    pub const JETBRAINSMONO_MEDIUMITALIC: &str = "fonts/JetBrainsMono-Medium-Italic.ttf";
    pub const JETBRAINSMONO_MEDIUM: &str = "fonts/JetBrainsMono-Medium.ttf";
    pub const JETBRAINSMONO_REGULAR: &str = "fonts/JetBrainsMono-Regular.ttf";
    pub const JETBRAINSMONO_SEMILIGHTITALIC: &str = "fonts/JetBrainsMono-SemiLight-Italic.ttf";
    pub const JETBRAINSMONO_SEMILIGHT: &str = "fonts/JetBrainsMono-SemiLight.ttf";
}
