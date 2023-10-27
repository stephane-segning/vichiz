import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { Room } from 'rust-tc-sdk';

type RoomState = Record<string, Room>;

const initialState: RoomState = {};

const roomSlice = createSlice({
  name: 'room',
  initialState,
  reducers: {
    setRoom: (state, action: PayloadAction<Room>) => {
      state[action.payload.id] = action.payload;
    },
  },
});

export const roomsSelector = (state: { room: RoomState }) =>
  Object.values(state.room);

export const roomSelector = (roomId: string) => (state: { room: RoomState }) =>
  state.room[roomId];

export const { setRoom } = roomSlice.actions;
export default roomSlice.reducer;
