// Disable no-unused-vars, broken for spread args
/* eslint no-unused-vars: off */
import { contextBridge, ipcRenderer } from 'electron';
import { Room } from '../renderer/models/room';

export type Channels = 'ipc-example';

const electronHandler = {
  encode: {
    encode(data: any, func: (...args: unknown[]) => void) {
      ipcRenderer.once('data-encode', (_event, ...args) => func(...args));
      ipcRenderer.send('data-encode', data);
    },
    decode(text: string, func: (...args: unknown[]) => void) {
      ipcRenderer.once('data-decode', (_event, ...args) => func(...args));
      ipcRenderer.send('data-decode', text);
    },
  },
  ataraxia: {
    async create(room: Room) {
      return ipcRenderer.sendSync('net-create', room);
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
    async destroy(roomId: string) {
      return ipcRenderer.sendSync('net-destroy');
    },
    async broadcast(roomId: string, type: string, data: any) {
      return ipcRenderer.sendSync('net-broadcast', type, data);
    },
  },
};

contextBridge.exposeInMainWorld('electron', electronHandler);

export type ElectronHandler = typeof electronHandler;

declare global {
  interface Window {
    electron: {
      encode: {
        encode: <T>(
          data: T,
          callback: (result: string, error?: any) => void,
        ) => void;
        decode: <T>(
          text: string,
          callback: (result: T, error?: any) => void,
        ) => void;
      };
      ataraxia: {
        create: (room: Room) => Promise<void>;
        onNodeAvailable: (
          roomId: string,
          callback: (node: any) => void,
        ) => void;
        onNodeUnavailable: (
          roomId: string,
          callback: (node: any) => void,
        ) => void;
        onMessage: (roomId: string, callback: (result: any) => void) => void;
        destroy: (roomId: string) => Promise<void>;
        broadcast: (roomId: string, type: string, data: any) => Promise<void>;
      };
    };
  }
}
