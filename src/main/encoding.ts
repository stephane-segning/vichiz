import * as pako from 'pako';
import { ipcMain } from 'electron';

interface EncodedData {
  data?: string;
  error?: any;
}

interface DecodedData<T> {
  data?: T;
  error?: any;
}

export function encodeData<T>(data: T): EncodedData {
  try {
    const strData = JSON.stringify(data); // Convert data to string
    const compressedData = pako.deflate(strData); // Compress the string
    return { data: btoa(compressedData as any) }; // Convert binary data to Base64 for sharing
  } catch (e) {
    console.error(e);
    return { error: e };
  }
}

export function decodeData<T>(encodedData: string): DecodedData<T> {
  try {
    const compressedData = atob(encodedData); // Convert Base64 to binary data
    const strData = pako.inflate(compressedData as any, { to: 'string' }); // Decompress the string
    return { data: JSON.parse(strData) };
  } catch (error) {
    console.error(error);
    return { error };
  }
}

ipcMain.on('data-encode', async (event, text: any) => {
  const { error, data } = encodeData(text);
  event.reply('data-encode', data, error);
});

ipcMain.on('data-decode', async (event, text: string) => {
  const { error, data } = decodeData(text);
  event.reply('data-decode', data, error);
});
