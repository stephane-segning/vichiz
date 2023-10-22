import { useNavigate, useParams } from 'react-router-dom';
import { LogOut, MoreHorizontal, Save } from 'react-feather';
import { useSelector } from 'react-redux';
import { useDPM } from '../hooks/dpm';
import { useVCM } from '../hooks/vcm';
import { roomSelector } from '../redux/room';

export function RoomPage() {
  const navigate = useNavigate();
  const { roomId } = useParams<{ roomId: string }>();
  const room = useSelector(roomSelector(roomId!));
  const { activeNodes } = useDPM(room);
  const { localStream } = useVCM(room);

  const exportRoomConfig = () => {};

  const logOut = () => {
    navigate('/');
  };

  return (
    <div>
      <div className="navbar bg-primary text-primary-content">
        <div className="flex-1">
          <span className="btn btn-ghost normal-case text-xl">{room.name}</span>
        </div>
        <div className="flex-none">
          <button
            type="button"
            onClick={logOut}
            className="btn btn-square btn-ghost">
            <LogOut />
          </button>
          <button
            type="button"
            onClick={exportRoomConfig}
            className="btn btn-square btn-ghost">
            <Save />
          </button>
          <button type="button" className="btn btn-square btn-ghost">
            <MoreHorizontal />
          </button>
        </div>
      </div>

      <h1>Room: {roomId}</h1>

      <div className="artboard artboard-horizontal phone-5">896×414</div>

      {/* Video, Audio, and Screen Sharing Components here */}
    </div>
  );
}
