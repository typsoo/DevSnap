use crate::handlers::{save, select_workspace};
use crate::storage_operations::storage_deleter;
use anyhow::Result;

pub fn handle_rewrite() -> Result<()> {
    let Some(name) = select_workspace("rewrite")? else {
        return Ok(());
    };

    storage_deleter::delete_snapshot(&name)?;
    println!("Selected workspace '{}' deleted for rewrite.", name);

    save::handle_save(name)?;

    Ok(())
}
