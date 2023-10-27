// Disable no-unused-vars, broken for spread args
/* eslint no-unused-vars: off */
import { contextBridge, ipcRenderer } from 'electron';
import type { Room, RoomOption } from 'rust-tc-sdk';

const electronHandler = {
  encode: {
    encode(data: any) {
      return ipcRenderer.invoke('data-encode', data);
    },
    decode(text: string) {
      return ipcRenderer.invoke('data-decode', text);
    },
  },
  sdk: {
    async create(room: RoomOption): Promise<Room> {
      return ipcRenderer.invoke('net-create', room);
    },
    async remove(roomId: string): Promise<void> {
      return ipcRenderer.invoke('net-remove', roomId);
    },
    async openRoom(roomId: string): Promise<void> {
      return ipcRenderer.invoke('net-open-room', roomId);
    },
    async closeRoom(roomId: string): Promise<void> {
      return ipcRenderer.invoke('net-close-room', roomId);
    },
    async getRooms(): Promise<Room[]> {
      return ipcRenderer.invoke('net-get-all');
    },
    async getRoom(id: string): Promise<Room> {
      return ipcRenderer.invoke('net-get-one', id);
    },
    async destroy() {
      return ipcRenderer.invoke('net-destroy');
    },
    async broadcast(roomId: string, type: string, data: any) {
      return ipcRenderer.invoke('net-broadcast', roomId, type, data);
    },
  },
};

contextBridge.exposeInMainWorld('electron', electronHandler);

export type ElectronHandler = typeof electronHandler;
