macro_rules! plugins {
    ( $( $x:ident ),* ) => {
        $(extern crate $x;)*

        pub fn init(config_dir: String) -> Vec<Box<ReceptorPlugin>> {
            let mut v: Vec<Box<ReceptorPlugin>> = vec!();
            $(
                match $x::new(config_dir.clone()) {
                    Ok(x) => {info!("{} successfully loaded", x.name()); v.push(Box::new(x)) }
                    Err(err) => {error!("Plugin: {}, error: {}", stringify!($x), err)}
                }
            )*
            v
        }
    }
}
