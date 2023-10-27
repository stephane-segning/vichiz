import { useEffect } from 'react';
import { Outlet } from 'react-router-dom';
import { useDispatch } from 'react-redux';
import { getRooms } from '../redux/room';

export function InitWrapper() {
  const dispatch = useDispatch();

  useEffect(() => {
    dispatch(getRooms() as any);
  }, [dispatch]);

  return <Outlet />;
}
