import React from 'react';
import {
  BrowserRouter as Router,
  Navigate,
  Route,
  Routes,
} from 'react-router-dom';
import { EnterRoomPage } from '../components/enter-room.page';
import { RoomPage } from '../components/room.page';

function Navigation() {
  return (
    <Router>
      <Routes>
        <Route>
          <Route path="/" Component={EnterRoomPage} />
          <Route path="/room/:roomId" Component={RoomPage} />
        </Route>

        <Route path="*" element={<Navigate to="/" />} />
      </Routes>
    </Router>
  );
}

export default Navigation;
