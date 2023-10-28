import {
  createRoom,
  getRoom,
  getRooms,
  launchRoom,
  quitRoom,
  removeRoom,
  RoomOption,
  startSdk,
  stopSdk,
} from 'rust-tc-sdk';
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

ipcMain.handle('net-get-all', async () => {
  return getRooms();
});

ipcMain.handle('net-get-one', async (_, id: string) => {
  return getRoom({ id });
});

ipcMain.handle('net-create', async (_, room: RoomOption) => {
  return createRoom(room);
});

ipcMain.handle('net-destroy', async () => {
  stopSdk(false);
});

ipcMain.handle('net-remove', async (_, roomId: string) => {
  removeRoom({ id: roomId });
});

ipcMain.handle('net-open-room', async (_, roomId: string) => {
  launchRoom({ room_id: roomId, room_listen_on: [], room_multi_address: [] });
});

ipcMain.handle('net-close-room', async (_, roomId: string) => {
  quitRoom({ id: roomId });
});
