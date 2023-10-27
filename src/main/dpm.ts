import { createRoom, RoomOption, startSdk, getRooms } from 'rust-tc-sdk';
import { app, ipcMain } from 'electron';
import * as path from 'path';
import * as fs from 'fs';

function dbPath() {
  const userDataPath = app.getPath('userData');
  const parentPath = path.join(userDataPath, 'db');
  fs.mkdirSync(parentPath, { recursive: true });
  return path.join(parentPath, 'database.db');
}

export function initDpm() {
  startSdk({
    db_url: dbPath(),
  });
}

ipcMain.on('net-get-all', async (event) => {
  const rooms = getRooms();
  event.reply('net-get-all', rooms);
});

ipcMain.on('net-create', async (event, room: RoomOption) => {
  const created = createRoom(room);
  console.log({ created, room });
  event.reply('net-create', created);
});

ipcMain.on('net-destroy', async (event, roomId) => {});
