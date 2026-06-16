const vscode = require('vscode');
const fs = require('fs');
const path = require('path');

function getDevSnapDir() {
    const home = process.env.HOME || process.env.USERPROFILE;
    const dir = path.join(home, '.config', 'devsnap', 'vscode_windows');
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    return dir;
}

function getFilePath() {
    return path.join(getDevSnapDir(), `${process.pid}.json`);
}

function saveState() {
    const folders = vscode.workspace.workspaceFolders;
    if (!folders || folders.length === 0) return;

    const folderPath = folders[0].uri.fsPath;
    
    fs.writeFileSync(getFilePath(), JSON.stringify({ folder: folderPath }));
}

function activate(context) {
    saveState();
    
    context.subscriptions.push(
        vscode.workspace.onDidChangeWorkspaceFolders(() => saveState())
    );
}

function deactivate() {
    const file = getFilePath();
    if (fs.existsSync(file)) {
        fs.unlinkSync(file);
    }
}

module.exports = {
    activate,
    deactivate
};