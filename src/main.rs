use invaders::Invaders;

fn main() -> anyhow::Result<()> {
    let mut invaders = Invaders::new()?;
    invaders.run();

    Ok(())
}
