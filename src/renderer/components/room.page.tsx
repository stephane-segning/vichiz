import { useNavigate, useParams, useSearchParams } from 'react-router-dom';
import { useSelector } from 'react-redux';
import { Clipboard, LogOut, MoreHorizontal } from 'react-feather';
import { useConnection } from '../hooks/connection';
import { hexConfigSelector } from '../redux/peer';

export function RoomPage() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const { roomId } = useParams<{ roomId: string }>();
  useConnection(roomId!, searchParams.get('hexConfig'));

  const hexConfix = useSelector(hexConfigSelector);

  const copyHexConfig = () => {
    if (!hexConfix) {
      return;
    }

    navigator.clipboard.writeText(hexConfix);
  };

  const logOut = () => {
    navigate('/');
  };

  return (
    <div>
      <div className="navbar bg-primary text-primary-content">
        <div className="flex-1">
          <span className="btn btn-ghost normal-case text-xl">{roomId}</span>
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
            onClick={copyHexConfig}
            className="btn btn-square btn-ghost">
            <Clipboard />
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
