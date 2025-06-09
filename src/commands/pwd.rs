use crate::ENV;
use crate::environment::ROOT_DIR;

pub fn pwd() -> anyhow::Result<()> {
    let root = ENV.get_root_path();
    let current = ENV.get_current_path();
    let relative = current.strip_prefix(root)?;

    let mut abs_path = format!("/{}", ROOT_DIR);

    if !relative.as_os_str().is_empty() {
        abs_path.push('/');
        abs_path.push_str(relative.to_str().unwrap());
    }

    println!("{}", abs_path);

    Ok(())
}
