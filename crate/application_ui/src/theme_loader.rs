use std::collections::HashMap;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::ui::{FontAsset, FontHandle, TtfFormat};
use application::resource::{self, dir, load_in};

use FontConfig;
use FontVariant;
use Theme;

/// Privates functionality to load an application theme.
#[derive(Debug)]
pub struct ThemeLoader;

impl ThemeLoader {
    /// Loads the theme into the `World`
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the theme into.
    pub fn load(world: &mut World) -> Result<(), resource::Error> {
        Self::load_internal(world, "font_config.ron")
    }

    #[inline]
    fn load_internal(world: &mut World, font_config_name: &str) -> Result<(), resource::Error> {
        let font_config: FontConfig = load_in(
            dir::RESOURCES,
            font_config_name,
            &resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        let font_paths = vec![
            (FontVariant::Regular, font_config.regular),
            (FontVariant::Bold, font_config.bold),
            (FontVariant::Italic, font_config.italic),
            (FontVariant::BoldItalic, font_config.bold_italic),
        ];

        let fonts = font_paths
            .into_iter()
            .map(|(font_variant, font_path)| {
                let loader = world.read_resource::<Loader>();
                let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
                let font_handle = loader.load(font_path, TtfFormat, (), (), &font_storage);
                (font_variant, font_handle)
            })
            .collect::<HashMap<FontVariant, FontHandle>>();

        world.add_resource(Theme::new(fonts));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::core::transform::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::renderer::ScreenDimensions;
    use amethyst::ui::UiBundle;
    use amethyst::Result;
    use application::resource;
    use strum::IntoEnumIterator;

    use super::ThemeLoader;
    use FontVariant;
    use Theme;

    fn setup<'a, 'b, F>(assertion_fn: Box<F>) -> Result<Application<'a, 'b>>
    where
        F: 'a + Fn(&mut World),
    {
        // We need to instantiate an amethyst::Application because:
        //
        // * The `Loader` needs to be added to the world, and the code to do this is non-trivial
        // * `Application` in amethyst does this
        Application::build(
            format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
            MockState { assertion_fn },
        )?.with_bundle(TransformBundle::new())?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_resource(ScreenDimensions::new(640, 480))
            .build()
    } // kcov-ignore

    #[test]
    fn build_adds_theme_with_fonts_to_world() {
        let assertion_fn = |world: &mut World| {
            ThemeLoader::load_internal(world, "font_config.ron").unwrap();

            let theme = world.read_resource::<Theme>();
            let fonts = &theme.fonts;
            debug!("Fonts: {:?}", &fonts);

            FontVariant::iter().for_each(|variant| assert!(fonts.contains_key(&variant)));
        };

        setup(Box::new(assertion_fn))
            .expect("Failed to build Application")
            .run(); // kcov-ignore
    }

    #[test]
    fn fails_with_useful_error_when_font_config_does_not_exist() {
        let assertion_fn = |world: &mut World| {
            if let Err(e) = ThemeLoader::load_internal(world, "non_existent.ron") {
                match *e.kind() {
                    resource::ErrorKind::Find(ref find_context) => {
                        assert_eq!("non_existent.ron", &find_context.file_name);
                        return; // pass
                    }
                    _ => {}
                }
            }

            panic!("Expected resource `Find` error containing `non_existent.ron`"); // kcov-ignore
        };

        setup(Box::new(assertion_fn))
            .expect("Failed to build Application")
            .run(); // kcov-ignore
    }

    #[test]
    fn fails_with_useful_error_when_font_config_fails_to_parse() {
        let assertion_fn = |world: &mut World| {
            if let Err(e) = ThemeLoader::load_internal(world, "bad_config.ron") {
                match *e.kind() {
                    resource::ErrorKind::RonDeserialization(ref _ron_error) => {
                        return; // pass
                    }
                    _ => {}
                }
            }

            panic!("Expected resource deserialization error"); // kcov-ignore
        };

        setup(Box::new(assertion_fn))
            .expect("Failed to build Application")
            .run(); // kcov-ignore
    }

    #[derive(Debug)]
    struct MockState<F: Fn(&mut World)> {
        assertion_fn: Box<F>,
    }
    impl<F: Fn(&mut World)> State for MockState<F> {
        fn on_start(&mut self, world: &mut World) {
            (self.assertion_fn)(world);
        }

        fn fixed_update(&mut self, _world: &mut World) -> Trans {
            Trans::Quit
        }
    }
}