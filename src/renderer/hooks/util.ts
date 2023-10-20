import { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';
import { connectionSelector } from '../redux/peer';

const RECONNECT_ATTEMPTS = 5;
const RECONNECT_INTERVAL = 5000; // 5 seconds

export function useReconnection() {
  const [reconnectAttempts, setReconnectAttempts] = useState(0);
  const connection = useSelector(connectionSelector);

  useEffect(() => {
    if (!connection) {
      if (reconnectAttempts < RECONNECT_ATTEMPTS) {
        setTimeout(() => {
          // Implement the code to try and re-establish the connection
          setReconnectAttempts((prev) => prev + 1);
        }, RECONNECT_INTERVAL);
      }
    }
  }, [connection, reconnectAttempts]);
}
