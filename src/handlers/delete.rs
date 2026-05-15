use crate::handlers::select_workspace;
use anyhow::Result;

pub fn handle_delete() -> Result<()> {
    let Some(name) = select_workspace("delete")? else {
        return Ok(());
    };

    crate::storage_operations::storage_deleter::delete_snapshot(&name)?;
    println!("Workspace '{}' has been successfully deleted.", name);

    Ok(())
}
