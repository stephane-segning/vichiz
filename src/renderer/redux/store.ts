import { configureStore } from '@reduxjs/toolkit';
import { combineReducers } from 'redux';
import storage from 'redux-persist/lib/storage'; // default: localStorage if web, AsyncStorage if react-native
import { persistReducer, persistStore } from 'redux-persist';

import { PersistConfig } from 'redux-persist/es/types';
import roomReducer from './room';
import connectionReducer from './connection';

const rootReducer = combineReducers({
  room: roomReducer,
  connection: connectionReducer,
});

const persistConfig: PersistConfig<any> = {
  key: 'root',
  storage,
  blacklist: ['peer'],
};

const persistedReducer = persistReducer(persistConfig, rootReducer);

const store = configureStore({
  reducer: persistedReducer,
});

export const persistor = persistStore(store);
export default store;
