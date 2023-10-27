import { useNavigate, useParams } from 'react-router-dom';
import {
  Camera,
  CameraOff,
  LogOut,
  Mic,
  MicOff,
  MoreHorizontal,
} from 'react-feather';
import { useSelector } from 'react-redux';
import { useEffect, useRef } from 'react';
import { useDPM } from '../hooks/dpm';
import { useVCM } from '../hooks/vcm';
import { roomSelector } from '../redux/room';

export function RoomPage() {
  const localVid = useRef<HTMLVideoElement>();
  const navigate = useNavigate();
  const { roomId } = useParams<{ roomId: string }>();
  const room = useSelector(roomSelector(roomId!));
  const { sendMessage } = useDPM(room);
  const {
    localStream,
    audioDevices,
    videoDevices,
    changeCamera,
    changeAudioDevice,
    isVideoOn,
    muteAudio,
    isAudioMuted,
    toggleVideo,
  } = useVCM(room);

  useEffect(() => {
    if (!localVid.current) return;

    localVid.current!.srcObject = localStream;
  }, [localStream]);

  const logOut = () => {
    navigate('/');
  };

  return (
    <div>
      <div className="navbar bg-primary text-primary-content">
        <div className="flex-1">
          <span className="btn btn-ghost normal-case text-xl">
            {room?.name}
          </span>
        </div>
        <div className="flex-none">
          <div className="dropdown dropdown-end">
            <button
              tabIndex={0}
              type="button"
              className="btn btn-square btn-ghost">
              {isVideoOn ? <Camera /> : <CameraOff />}
            </button>

            <div className="mt-3 z-[1] card card-compact dropdown-content w-52 bg-primary shadow">
              <div className="card-body">
                {videoDevices.map((device) => (
                  <div key={device.deviceId}>
                    <button
                      type="button"
                      onClick={() => changeCamera(device.deviceId)}
                      className="btn btn-ghost btn-block">
                      {device.label}
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </div>

          <div className="dropdown dropdown-end">
            <button
              tabIndex={0}
              type="button"
              className="btn btn-square btn-ghost">
              {isAudioMuted ? <MicOff /> : <Mic />}
            </button>

            <div className="mt-3 z-[1] card card-compact dropdown-content w-56 bg-primary shadow">
              <div className="card-body">
                {audioDevices.map((device) => (
                  <div key={device.deviceId}>
                    <button
                      type="button"
                      onClick={() => changeAudioDevice(device.deviceId)}
                      className="btn btn-ghost btn-block">
                      {device.label}
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </div>

          <button
            type="button"
            onClick={logOut}
            className="btn btn-square btn-error">
            <LogOut />
          </button>
          <button type="button" className="btn btn-square btn-ghost">
            <MoreHorizontal />
          </button>
        </div>
      </div>

      <div className="artboard artboard-horizontal phone-5">
        <video ref={localVid} className="h-full w-full" autoPlay muted />
      </div>
    </div>
  );
}
