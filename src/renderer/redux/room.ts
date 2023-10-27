import { createAsyncThunk, createSlice, PayloadAction } from '@reduxjs/toolkit';
import { Room, RoomOption } from 'rust-tc-sdk';

type RoomState = Record<string, Room>;

const initialState: RoomState = {};

export const ROOM_TOKEN = 'room';

export const getRooms = createAsyncThunk(
  `${ROOM_TOKEN}/getRooms`,
  async (args, thunkAPI) => {
    const rooms = await window.electron.sdk.getRooms();
    return rooms;
  },
);

export const getRoom = createAsyncThunk(
  `${ROOM_TOKEN}/getRoom`,
  async (id: string, thunkAPI) => {
    const room = await window.electron.sdk.getRoom(id);
    return room;
  },
);

export const createRoom = createAsyncThunk(
  `${ROOM_TOKEN}/createRoom`,
  async (option: RoomOption, thunkAPI) => {
    const room = await window.electron.sdk.create(option);
    return room;
  },
);

export const removeRoom = createAsyncThunk(
  `${ROOM_TOKEN}/removeRoom`,
  async (roomId: string, thunkAPI) => {
    await window.electron.sdk.remove(roomId);
  },
);

export interface BroadcastPayload {
  roomId: string;
  type: string;
  data: any;
}

export const broadcast = createAsyncThunk(
  `${ROOM_TOKEN}/broadcast`,
  async ({ data, type, roomId }: BroadcastPayload, thunkAPI) => {
    await window.electron.sdk.broadcast(roomId, type, data);
  },
);

const roomSlice = createSlice({
  name: ROOM_TOKEN,
  initialState,
  reducers: {
    setRoom: (state, action: PayloadAction<Room>) => {
      state[action.payload.id] = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(getRooms.fulfilled, (state, action) => {
      action.payload.forEach((room) => {
        state[room.id] = room;
      });
    });

    builder.addCase(getRoom.fulfilled, (state, action) => {
      state[action.payload.id] = action.payload;
    });

    builder.addCase(createRoom.fulfilled, (state, action) => {
      state[action.payload.id] = action.payload;
    });

    builder.addCase(removeRoom.fulfilled, (state, action) => {
      delete state[action.meta.arg];
    });

    builder.addCase(broadcast.fulfilled, (state, action) => {
      // Do nothing
    });
  },
});

export const roomsSelector = (state: { room: RoomState }) => state.room;

export const roomSelector = (roomId: string) => (state: { room: RoomState }) =>
  state.room[roomId];

export const { setRoom } = roomSlice.actions;
export default roomSlice.reducer;
