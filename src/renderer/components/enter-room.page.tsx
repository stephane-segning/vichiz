import { useState } from 'react';
import { useSelector } from 'react-redux';
import { roomsSelector } from '../redux/room';
import { usePrepareRoom } from '../hooks/connection';
import icon from '../../../assets/icon.svg';

export function EnterRoomPage() {
  const [roomId, setRoomId] = useState('');
  const prepareRoom = usePrepareRoom();
  const rooms = useSelector(roomsSelector);

  const handleSubmit = () => {
    if (roomId) {
      prepareRoom(roomId.trim());
    }
  };

  return (
    <div className="w-full max-w-md px-4 mx-auto py-16">
      <img className="pb-4 mx-auto" width="144" alt="icon" src={icon} />
      <h1 className="text-4xl pb-4 font-bold text-center">Enter Room</h1>
      <div className="pb-2">
        <input
          className="input input-bordered w-full"
          type="text"
          placeholder="Enter Room ID"
          value={roomId}
          onChange={(e) => setRoomId(e.target.value)}
        />
      </div>
      <button type="button" className="btn btn-block" onClick={handleSubmit}>
        Join Room
      </button>
      <div className="divider" />
      <div>
        {rooms.map((room) => (
          <button
            type="button"
            onClick={() => prepareRoom(room)}
            key={room}
            className="kbd kbd-md mr-2 mb-2">
            {room}
          </button>
        ))}
      </div>
    </div>
  );
}
