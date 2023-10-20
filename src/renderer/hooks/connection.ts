import { useDispatch } from 'react-redux';
import SimplePeer from 'simple-peer';
import { useCallback, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  clearConnection,
  setConnection,
  setError,
  setHexConfig,
  setRoomId,
} from '../redux/peer';
import { addRoomId } from '../redux/room';
import { decodeFromHex, encodeToHex } from '../utils/encoding';

interface EncodedConfig {
  signalData: SimplePeer.SignalData;
}

// To use on the enter room page
export function usePrepareRoom() {
  const navigate = useNavigate();
  return (roomId: string, offerHex?: string) => {
    if (offerHex) {
      navigate(`/room/${roomId}`, {
        state: { offerHex },
      });
    } else {
      navigate(`/room/${roomId}`);
    }
  };
}

export function useConnection(roomId: string, hexConfig?: string | null) {
  const navigate = useNavigate();
  const dispatch = useDispatch();

  const startConnection = useCallback(() => {
    dispatch(setRoomId(roomId));

    try {
      const config = hexConfig ? decodeFromHex<EncodedConfig>(hexConfig) : null;

      const peer = new SimplePeer({
        initiator: !config,
        trickle: false,
      });

      peer.on('signal', (data) => {
        console.log('SIGNAL', data);
        // Send this data to the other peer manually (e.g., via QR code or text)
        const newHexConfig = encodeToHex<EncodedConfig>({ signalData: data });
        dispatch(setHexConfig(newHexConfig));
      });

      if (config) {
        peer.signal(config.signalData);
      }

      dispatch(setConnection(peer));
      dispatch(addRoomId(roomId));
    } catch (error) {
      dispatch(setError('Error setting up the connection.'));
    }
  }, [dispatch, hexConfig, roomId]);

  const endConnection = useCallback(
    (shouldNavigate = false) => {
      dispatch(clearConnection());

      if (shouldNavigate) {
        navigate('/');
      }
    },
    [dispatch, navigate],
  );

  // Automatically try to start the connection when the page is loaded with a roomId
  useEffect(() => {
    if (roomId) {
      startConnection();
    }
    // Cleanup on component unmount
    return () => {
      endConnection();
    };
  }, [roomId, hexConfig, startConnection, endConnection]);

  return { startConnection, endConnection };
}
