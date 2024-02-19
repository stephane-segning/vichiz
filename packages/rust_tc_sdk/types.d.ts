declare module 'rust-tc-sdk' {
  export function startSdk(options: RustSDKOptions): Promise<void>;

  export function stopSdk(cleanUp: boolean): Promise<void>;

  export function createRoom(option: RoomOption): Promise<Room>;

  export function removeRoom(data: RoomId): Promise<void>;

  export function launchRoom(data: ConnectionData): Promise<void>;

  export function getRoom(data: RoomId): Promise<Room>;

  export function quitRoom(data: RoomId): Promise<void>;

  export function getRooms(): Promise<Room[]>;

  export function registerListener(callback: Callback): Promise<void>;

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
