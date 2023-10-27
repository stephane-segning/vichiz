import './App.css';

import { Provider as ReduxProvider } from 'react-redux';
import store from './redux/store';
import Navigation from './router/navigation';

export default function App() {
  return (
    <ReduxProvider store={store}>
      <Navigation />
    </ReduxProvider>
  );
}
