import { startSdk } from 'rust-tc-sdk';
import { ipcMain } from 'electron';

export function initDpm() {
  startSdk('./app.db');
}

ipcMain.on('net-create', async (event, room) => {});

ipcMain.on('net-destroy', async (event, roomId) => {});
