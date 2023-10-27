import { configureStore } from '@reduxjs/toolkit';
import { combineReducers } from 'redux';

import roomReducer from './room';
import connectionReducer from './connection';

const rootReducer = combineReducers({
  room: roomReducer,
  connection: connectionReducer,
});

const store = configureStore({
  reducer: rootReducer,
});

export default store;
