import * as pako from 'pako';

interface EncodedData {
  data?: string;
  error?: any;
}

interface DecodedData {
  data?: any;
  error?: any;
}

export function encodeData<T>(data: T): EncodedData {
  try {
    const strData = JSON.stringify(data); // Convert data to string
    const compressedData = pako.deflate(strData, { to: 'string' }); // Compress the string
    return { data: btoa(compressedData) }; // Convert binary data to Base64 for sharing
  } catch (e) {
    console.error(e);
    return { error: e };
  }
}

export function decodeData<T>(encodedData: string): DecodedData {
  try {
    const compressedData = atob(encodedData); // Convert Base64 to binary data
    const strData = pako.inflate(compressedData, { to: 'string' }); // Decompress the string
    return { data: JSON.parse(strData) };
  } catch (error) {
    console.error(error);
    return { error };
  }
}
