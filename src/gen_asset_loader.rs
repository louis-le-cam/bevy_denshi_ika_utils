#[macro_export]
macro_rules! gen_asset_loader {
    (
        $name:ident<$asset:ty, $settings_ty:ty, $error:ty> $extensions:expr;
        |&self, $reader:ident, $settings:ident, $load_context:ident| {
            $($body:tt)*
        }
    ) => {
        #[derive(Default)]
        pub struct $name;

        impl bevy::asset::AssetLoader for $name {
            type Asset = $asset;
            type Settings = $settings_ty;
            type Error = $error;

            fn load<'a>(
                &'a self,
                $reader: &'a mut bevy::asset::io::Reader,
                $settings: &'a Self::Settings,
                $load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
                $($body)*
            }

            fn extensions(&self) -> &[&str] {
                &$extensions
            }
        }

    };
}
