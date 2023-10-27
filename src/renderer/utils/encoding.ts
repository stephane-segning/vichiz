type Maybe<T> = T | undefined;

export const encodeToHex = async <T>(data: T): Promise<string> => {
  const { error, result } = await window.electron.encode.encode<T>(data);
  if (error) {
    throw error;
  }
  return result;
};

export const decodeFromHex = async <T>(hex: string): Promise<Maybe<T>> => {
  const { error, result } = await window.electron.encode.decode<T>(hex);
  if (error) {
    throw error;
  }
  return result;
};
