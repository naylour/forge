use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn run() -> Result<()> {
    let key_path = get_key_path()?;

    if !key_path.exists() {
        generate_key(&key_path)?;
    }

    print_public_key(&key_path)?;

    Ok(())
}

fn get_key_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Не удалось определить директорию конфига")?;

    Ok(config_dir.join("age").join("ingot-secret.txt"))
}

fn generate_key(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Не удалось создать директорию для ключа")?;
    }

    let status = std::process::Command::new("age-keygen")
        .args(["-o", path.to_str().context("некорректный путь")?])
        .status()
        .context("не удалось запустить age-keygen")?;

    anyhow::ensure!(status.success(), "age-keygen завершился с ошибкой");

    Ok(())
}

fn print_public_key(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path).context("не удалось прочитать файл ключа")?;

    let public_key = content
        .lines()
        .find(|line| line.starts_with("# public key:"))
        .context("побличный ключ не найден в файле")?
        .trim_start_matches("# public key:")
        .trim();

    println!("Твой публичный ключ:");
    println!("{}", public_key);
    println!();
    println!("Отправь его владельцу проекта.");

    Ok(())
}
