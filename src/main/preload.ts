// Disable no-unused-vars, broken for spread args
/* eslint no-unused-vars: off */
import { contextBridge, ipcRenderer } from 'electron';
import { Room, RoomOption } from 'rust-tc-sdk';

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
    async getRooms(): Promise<Room[]> {
      return ipcRenderer.invoke('net-get-all');
    },
    async getRoom(id: string): Promise<Room> {
      return ipcRenderer.invoke('net-get-one', id);
    },
    onNodeAvailable(roomId: string, callback: (...args: unknown[]) => void) {
      ipcRenderer.on(`net-node-available-${roomId}`, (_event, ...args) =>
        callback(...args),
      );
    },
    onNodeUnavailable(roomId: string, callback: (...args: unknown[]) => void) {
      ipcRenderer.on(`net-node-unavailable-${roomId}`, (_event, ...args) =>
        callback(...args),
      );
    },
    onMessage(roomId: string, callback: (...args: unknown[]) => void) {
      ipcRenderer.on(`net-message-${roomId}`, (_event, ...args) =>
        callback(...args),
      );
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

declare global {
  interface Window {
    electron: {
      encode: {
        encode: <T>(data: T) => Promise<{ result: string; error?: any }>;
        decode: <T>(text: string) => Promise<{ result: T; error?: any }>;
      };
      sdk: {
        create(room: RoomOption): Promise<Room>;
        remove(roomId: string): Promise<void>;
        getRooms(): Promise<Room[]>;
        getRoom(id: string): Promise<Room>;
        onNodeAvailable(roomId: string, callback: (node: any) => void): void;
        onNodeUnavailable(roomId: string, callback: (node: any) => void): void;
        onMessage(roomId: string, callback: (result: any) => void): void;
        destroy(roomId: string): Promise<void>;
        broadcast(roomId: string, type: string, data: any): Promise<void>;
      };
    };
  }
}
