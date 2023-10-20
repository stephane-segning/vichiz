import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import SimplePeer from 'simple-peer';

interface PeerState {
  connection: SimplePeer.Instance | null;
  config: SimplePeer.SignalData | null;
  hexConfig: string | null;
  error: string | null;
  roomId: string | null;
}

const initialState: PeerState = {
  connection: null,
  config: null,
  hexConfig: null,
  error: null,
  roomId: null,
};

const peerSlice = createSlice({
  name: 'peer',
  initialState,
  reducers: {
    setConnection: (state, action: PayloadAction<SimplePeer.Instance>) => {
      state.connection = action.payload;
      state.error = null;
    },
    setError: (state, action: PayloadAction<string>) => {
      state.error = action.payload;
    },
    setRoomId: (state, action: PayloadAction<string>) => {
      state.roomId = action.payload;
    },
    clearConnection: (state) => {
      console.log('clearConnection');
      if (state.connection) {
        try {
          state.connection.end();
          state.connection.destroy();
        } catch (error) {
          console.warn(error);
        }
      }

      console.log('clearConnection2');

      state.connection = null;
      state.config = null;
      state.hexConfig = null;
      state.roomId = null;
    },
    setHexConfig: (state, action: PayloadAction<string>) => {
      state.hexConfig = action.payload;
    },
    setConfig: (state, action: PayloadAction<SimplePeer.SignalData>) => {
      state.config = action.payload;
    },
  },
});

export const configSelector = (state: { peer: PeerState }) => state.peer.config;
export const hexConfigSelector = (state: { peer: PeerState }) =>
  state.peer.hexConfig;
export const roomIdConfigSelector = (state: { peer: PeerState }) =>
  state.peer.roomId;
export const connectionSelector = (state: {
  peer: PeerState;
}): SimplePeer.Instance | null => state.peer.connection;

export const {
  setConfig,
  setHexConfig,
  setConnection,
  setError,
  setRoomId,
  clearConnection,
} = peerSlice.actions;

export default peerSlice.reducer;
