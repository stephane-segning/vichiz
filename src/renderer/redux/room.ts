import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import * as _ from 'lodash';

interface RoomState {
  allRoomIds: string[];
}

const initialState: RoomState = {
  allRoomIds: [],
};

const roomSlice = createSlice({
  name: 'room',
  initialState,
  reducers: {
    addRoomId: (state, action: PayloadAction<string>) => {
      const prev = state.allRoomIds;
      state.allRoomIds = _.uniq([...prev, action.payload]);
    },
  },
});

export const roomsSelector = (state: { room: RoomState }) =>
  state.room.allRoomIds;

export const { addRoomId } = roomSlice.actions;
export default roomSlice.reducer;
