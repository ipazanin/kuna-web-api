use super::models::config::Config;

pub fn create_config() -> Config {
    let mut config = create_config_from_settings_file();

    modify_config_with_values_from_env_variables(&mut config);

    config
}

fn create_config_from_settings_file() -> Config {
    let config_file_text = include_str!("settings.json");

    let config =
        serde_json::from_str::<Config>(config_file_text).expect("Invalid JSON configuration file!");

    println!("Deserialized config: {:?}", config);

    config
}

fn modify_config_with_values_from_env_variables(config: &mut Config) {
    config.set_from_environment();

    println!("Config after environment variables: {:?}", config);
}
