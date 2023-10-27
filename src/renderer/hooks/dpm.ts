import { useCallback, useEffect } from 'react';
import type { Room } from 'rust-tc-sdk';
import { useDispatch } from 'react-redux';
import { broadcast, closeRoom, openRoom } from '../redux/room';

export const useDPM = (room?: Room) => {
  const dispatch = useDispatch();
  const sendMessage = useCallback(
    async (message: string, data: any) => {
      if (!room?.id) return;

      // Send a message to peers
      dispatch(broadcast({ roomId: room.id, type: message, data }) as any);
    },
    [dispatch, room?.id],
  );

  useEffect(() => {
    if (!room?.id) return () => {};

    dispatch(openRoom(room?.id) as any);
    return () => {
      dispatch(closeRoom(room?.id) as any);
    };
  }, [dispatch, room?.id]);

  return { sendMessage };
};
