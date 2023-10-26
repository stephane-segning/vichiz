declare module 'rust-tc-sdk' {
  export function startSdk(dbUrl: string): void;

  export function stopSdk(cleanUp: boolean): void;

  export function createRoom(option: RoomOption): void;

  export function launchRoom(data: ConnectionData): void;

  export interface ConnectionData {
    roomId: string;
    roomMultiAddress?: string[];
    roomListen_on?: string[];
  }

  export interface RoomOption {
    id?: string;
    name: string;
  }

  export interface Room {
    id: string;
    name: string;
  }

  export interface NoiseModel {
    id: string;
    public: string;
  }
}
