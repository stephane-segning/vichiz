import { useCallback, useEffect, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { setRoom } from '../redux/room';
import {
  currentHostSelector,
  nextHostSelector,
  setCurrentHost,
  setNextHost,
  setRoomId,
} from '../redux/connection';
import { Room } from '../models/room';

interface PeerScores {
  [peerId: string]: number;
}

interface PCPerformanceSpecs {
  cpuPerformance: number;
  availableMemory: number;
  gpuPerformance: number;
  batteryLevel: number;
}

const getPCPerformanceSpecs = (): PCPerformanceSpecs => {
  // Logic to get PC performance specs.
  return {
    cpuPerformance: 0.5,
    availableMemory: 0.5,
    gpuPerformance: 0.5,
    batteryLevel: 0.5,
  };
};

const determineNextHost = (peerScores: PeerScores): string => {
  // Logic to determine the next host based on peer scores.
  return Object.keys(peerScores).reduce((a, b) =>
    peerScores[a] > peerScores[b] ? a : b,
  );
};

const calculateScore = (data: PCPerformanceSpecs): number => {
  const weights = {
    cpuPerformance: 0.4,
    availableMemory: 0.3,
    gpuPerformance: 0.2,
    batteryLevel: 0.1,
  };

  return (
    data.cpuPerformance * weights.cpuPerformance +
    data.availableMemory * weights.availableMemory +
    data.gpuPerformance * weights.gpuPerformance +
    data.batteryLevel * weights.batteryLevel
  );
};

// Discovery and Peer Management Hook
export const useDPM = (room: Room) => {
  const dispatch = useDispatch();
  const localPeerId = useSelector(currentHostSelector);
  const nextHost = useSelector(nextHostSelector);
  const [error, setError] = useState<Error | null>(null);
  const [messages, setMessages] = useState<string[]>([]);
  const [activeNodes, setActiveNodes] = useState<string[]>([]);
  const [peerScores, setPeerScores] = useState<PeerScores>({});
  const [specs, setSpecs] = useState<PCPerformanceSpecs | null>(null);

  const sendMessage = useCallback(
    async (message: string, data: any) => {
      // Send a message to peers
      await window.electron.ataraxia.broadcast(room.id, message, data);
    },
    [room.id],
  );

  useEffect(() => {
    const interval = setInterval(
      async () => {
        const data = getPCPerformanceSpecs();
        setSpecs(data);
        await sendMessage('SPEC_DATA', data);
      },
      10 * 60 * 1000,
    ); // Every 10 minutes

    return () => clearInterval(interval as any);
  }, [sendMessage]);

  useEffect(() => {
    let cancel: () => Promise<void> = async () => {};

    const init = async () => {
      await window.electron.ataraxia.create(room);

      window.electron.ataraxia.onNodeAvailable(room.id, async (node) => {
        await node.send('ROOM_INFO', room);
        setActiveNodes((prev) => [...prev, node.id]);
      });

      window.electron.ataraxia.onNodeUnavailable(room.id, (node) => {
        // Handle a node leaving or becoming unavailable.
        setActiveNodes((prev) => prev.filter((id) => id !== node.id));
      });

      window.electron.ataraxia.onMessage(room.id, (msg) => {
        if (msg.type === 'ROOM_INFO') {
          // Store the room information in the Redux store.
          dispatch(setRoom(msg.data));
        } else if (msg.type === 'NEXT_HOST_UPDATE') {
          dispatch(setNextHost(msg.data));
        } else if (msg.type === 'HOST_GOODBYE') {
          if (localPeerId === nextHost) {
            // This peer is the next host.
            dispatch(setCurrentHost(localPeerId!));
          }
        } else if (msg.type === 'SPEC_DATA') {
          const score = calculateScore(msg.data);
          setPeerScores((prevState) => ({
            ...prevState,
            [msg.source.id]: score,
          }));
        }
      });

      dispatch(setRoomId(room.id));

      cancel = async () => {
        if (localPeerId === nextHost) {
          const nextHostId = determineNextHost(peerScores);
          // Send this information to all peers
          await sendMessage('NEXT_HOST_UPDATE', nextHostId);
          // Store this locally as well
          dispatch(setNextHost(nextHostId));
        }

        await window.electron.ataraxia.destroy(room.id);
      };
    };

    init();

    return () => {
      cancel();
    };
  }, [dispatch, localPeerId, nextHost, peerScores, room, sendMessage]);

  return { error, sendMessage, messages, activeNodes, specs };
};
