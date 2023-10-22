export interface RoomMember {
  id: string;
  nodeId: string;
}

export interface Room {
  id: string;
  name: string;
  secretToken: string;
}
