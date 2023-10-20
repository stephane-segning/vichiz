import './App.css';

import { Provider as ReduxProvider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';
import store, { persistor } from './redux/store';
import Navigation from './router/navigation';
import { SplashScreen } from './components/splash.screen';

export default function App() {
  return (
    <ReduxProvider store={store}>
      <PersistGate loading={<SplashScreen />} persistor={persistor}>
        <Navigation />
      </PersistGate>
    </ReduxProvider>
  );
}
