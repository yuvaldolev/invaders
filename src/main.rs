use invaders::Invaders;

fn main() -> anyhow::Result<()> {
    // Initialize the logger according to the environment.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // Run the game.
    let mut invaders = Invaders::new()?;
    invaders.run();

    Ok(())
}
