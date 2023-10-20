import { Buffer } from 'buffer';

type Maybe<T> = T | undefined;

export const encodeToHex = <T>(data: T): Promise<string> => {
  const str = JSON.stringify(data);
  return Buffer.from(str).toString('base64');
};

export const decodeFromHex = <T>(hex: string): Promise<Maybe<T>> => {
  try {
    return new Promise((resolve, reject) => {
      window.electron.encode.decode<T>(hex, (result, error) => {
        if (error) {
          reject(error);
          return;
        }
        resolve(result);
      });
    });
  } catch (error) {
    console.error(error);
    return Promise.resolve(undefined);
  }
};
