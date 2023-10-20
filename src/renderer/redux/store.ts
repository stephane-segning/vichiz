import { configureStore } from '@reduxjs/toolkit';
import { combineReducers } from 'redux';
import storage from 'redux-persist/lib/storage'; // default: localStorage if web, AsyncStorage if react-native
import { persistReducer, persistStore } from 'redux-persist';

import { PersistConfig } from 'redux-persist/es/types';
import roomReducer from './room';
import peerReducer from './peer';

const rootReducer = combineReducers({
  room: roomReducer,
  peer: peerReducer,
  // ... other reducers
});

const persistConfig: PersistConfig<any> = {
  key: 'root',
  storage,
  whitelist: ['room'], // only persist 'room'
  blacklist: ['peer'], // do not persist 'peer'
};

const persistedReducer = persistReducer(persistConfig, rootReducer);

const store = configureStore({
  reducer: persistedReducer,
});

export const persistor = persistStore(store);
export default store;
