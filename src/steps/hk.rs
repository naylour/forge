use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    which::which("hk").context("hk не найден в PATH")?;

    let status = std::process::Command::new("hk")
        .arg("install")
        .status()
        .context("не удалось запустить hk install")?;

    anyhow::ensure!(status.success(), "hk install завершился с ошибкой");

    Ok(())
}
