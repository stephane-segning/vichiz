declare module 'rust-tc-sdk' {
  export function startSdk(options: RustSDKOptions): void;

  export function stopSdk(cleanUp: boolean): void;

  export function createRoom(option: RoomOption): void;

  export function launchRoom(data: ConnectionData): void;

  export interface RustSDKOptions {
    db_url?: string;
  }

  export interface ConnectionData {
    room_id: string;
    room_multi_address?: string[];
    room_listen_on?: string[];
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
