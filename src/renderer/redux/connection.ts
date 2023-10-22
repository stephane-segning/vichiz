import { createSlice, PayloadAction } from '@reduxjs/toolkit';

type ConnectionState = {
  currentHost?: string;
  nextHost?: string;
  roomId?: string;
};

const initialState: ConnectionState = {};

const connectionSlice = createSlice({
  name: 'connection',
  initialState,
  reducers: {
    setCurrentHost: (state, action: PayloadAction<string>) => {
      state.currentHost = action.payload;
    },
    setNextHost: (state, action: PayloadAction<string>) => {
      state.nextHost = action.payload;
    },
    setRoomId: (state, action: PayloadAction<string>) => {
      state.roomId = action.payload;
    },
  },
});

export const currentHostSelector = (state: { room: ConnectionState }) =>
  state.room.currentHost;
export const nextHostSelector = (state: { room: ConnectionState }) =>
  state.room.nextHost;

export const { setCurrentHost, setRoomId, setNextHost } =
  connectionSlice.actions;
export default connectionSlice.reducer;
