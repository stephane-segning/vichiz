// Disable no-unused-vars, broken for spread args
/* eslint no-unused-vars: off */
import { contextBridge, ipcRenderer } from 'electron';

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
    };
  }
}
