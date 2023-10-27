declare module 'rust-tc-sdk' {
  export function startSdk(options: RustSDKOptions): void;

  export function stopSdk(cleanUp: boolean): void;

  export function createRoom(option: RoomOption): Room;

  export function removeRoom(data: RoomId): void;

  export function launchRoom(data: ConnectionData): void;

  export function getRoom(data: RoomId): Room;

  export function quitRoom(data: RoomId): void;

  export function getRooms(): Room[];

  export function registerListener(callback: Callback): void;

  export type Callback = (type: string, data: CallbackPayload) => void;

  export interface CallbackPayload {
    data: string;
  }

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

  export interface RoomId {
    id: string;
  }
}
