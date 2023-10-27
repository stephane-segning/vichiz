import { useCallback } from 'react';
import { Room } from 'rust-tc-sdk';
import { useDispatch } from 'react-redux';
import { broadcast } from '../redux/room';

export const useDPM = (room: Room) => {
  const dispatch = useDispatch();
  const sendMessage = useCallback(
    async (message: string, data: any) => {
      // Send a message to peers
      dispatch(broadcast({ roomId: room.id, type: message, data }) as any);
    },
    [dispatch, room.id],
  );

  return { sendMessage };
};
