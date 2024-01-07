#[macro_export]
macro_rules! gen_plugin {
    (
        $vis:vis $name:ident $(: $($plugins:expr),+ $(,)?)?;
        $(reflect<$($reflect_type:ty),* $(,)?>;)?
        $(assets <$($assets:ty),* $(,)?>;)?
        $(asset_loaders<$($asset_loaders:ty),* $(,)?>;)?
        $(states<$($states:ty),* $(,)?>;)?
        $(init_resources<$($init_resources:ty),* $(,)?>;)?
        $(systems($schedule:expr $(, $state:expr)?) $systems:expr;)*
    ) => {
        gen_plugin!{
            $vis $name $(: $($plugins),+)?;
            $(reflect<$($reflect_type),*>;)?
            $(assets<$($assets),*>;)?
            $(asset_loaders<$($asset_loaders),*>;)?
            $(states<$($states),*>;)?
            $(init_resources<$($init_resources),*>;)?
            $(systems ($schedule $(, $state)?) $systems;)*
            |app| {}
        }
    };

    (
        $vis:vis $name:ident $(: $($plugins:expr),+ $(,)?)?;
        $(reflect<$($reflect_type:ty),* $(,)?>;)?
        $(assets <$($assets:ty),* $(,)?>;)?
        $(asset_loaders<$($asset_loaders:ty),* $(,)?>;)?
        $(states<$($states:ty),* $(,)?>;)?
        $(init_resources<$($init_resources:ty),* $(,)?>;)?
        $(systems($schedule:expr $(, $state:expr)?) $systems:expr;)*
        | $app:ident | $code: block
    ) => {

        $vis struct $name;
        impl bevy::prelude::Plugin for $name {
            fn build(&self, #[allow(unused_variables)] $app: &mut bevy::prelude::App) {
                #[allow(unused_imports)]
                use bevy::prelude::*;

                $($app.add_plugins(($($plugins),*));)?
                $($(
                    $app.register_type::<$reflect_type>();
                )*)?
                $($($app.init_asset::<$assets>();)*)?
                $($($app.init_asset_loader::<$asset_loaders>();)*)?
                $($($app.add_state::<$states>();)*)?
                $($app.add_systems($schedule, ($systems) $(.run_if(in_state($state)))?);)*
                $($($app.init_resource::<$init_resources>();)*)?

                $code;
            }
        }
    };
}
