import {
  createRoom,
  getRoom,
  getRooms,
  RoomOption,
  startSdk,
  stopSdk,
  quitRoom,
  launchRoom,
  removeRoom,
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

ipcMain.handle('net-get-all', async (event) => {
  const rooms = getRooms();
  return rooms;
});

ipcMain.handle('net-get-one', async (event, id: string) => {
  const room = getRoom({ id });
  return room;
});

ipcMain.handle('net-create', async (event, room: RoomOption) => {
  const created = createRoom(room);
  return created;
});

ipcMain.handle('net-destroy', async (event) => {
  stopSdk(false);
});

ipcMain.handle('net-start-room', async (event, roomId: string) => {
  launchRoom({ room_id: roomId });
});

ipcMain.handle('net-quit-room', async (event, roomId: string) => {
  quitRoom({ id: roomId });
});

ipcMain.handle('net-remove', async (event, roomId: string) => {
  removeRoom({ id: roomId });
});
