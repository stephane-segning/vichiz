import { configureStore } from '@reduxjs/toolkit';
import { combineReducers } from 'redux';

import roomReducer, { ROOM_TOKEN } from './room';
import connectionReducer from './connection';

const rootReducer = combineReducers({
  [ROOM_TOKEN]: roomReducer,
  connection: connectionReducer,
});

const store = configureStore({
  reducer: rootReducer,
});

export default store;
