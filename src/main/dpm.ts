import {
  createRoom,
  getRoom,
  getRooms,
  launchRoom,
  quitRoom,
  registerListener,
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

export async function initDpm() {
  await startSdk({
    db_url: dbPath(),
  });

  await registerListener((type, data) => {
    console.log('>>> test', type, data);
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
  await stopSdk(false);
});

ipcMain.handle('net-remove', async (_, roomId: string) => {
  await removeRoom({ id: roomId });
});

ipcMain.handle('net-open-room', async (_, roomId: string) => {
  await launchRoom({
    room_id: roomId,
    room_listen_on: [],
    room_multi_address: [],
  });
});

ipcMain.handle('net-close-room', async (_, roomId: string) => {
  await quitRoom({ id: roomId });
});
