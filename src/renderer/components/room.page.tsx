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
import { useCallback, useEffect, useRef } from 'react';
import { useDPM } from '../hooks/dpm';
import { useVCM } from '../hooks/vcm';
import { roomSelector } from '../redux/room';
import { VideoDisplay } from './video-display';
import VideoCard from './video-block';

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
    videoDevice,
  } = useVCM(room);
  const otherStreams = [
    localStream,
    localStream,
    localStream,
    localStream,
    localStream,
  ];

  useEffect(() => {
    if (!localStream || !localVid.current) return () => {};

    const videoElement = localVid.current!;
    videoElement.srcObject = localStream;
    // videoElement.play();
    console.log('localStream', localStream);
    return () => {
      videoElement.srcObject = null;
    };
  }, [localStream, localVid]);

  const handleStreamClick = useCallback((stream: MediaStream) => {
    // TODO: handle stream click
  }, []);

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
                      onClick={() => changeCamera(device)}
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
                      onClick={() => changeAudioDevice(device)}
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

      <div className="w-screen h-[calc(100vh-64px)] relative">
        <div className="absolute inset-0">
          <VideoDisplay stream={localStream} />
        </div>
        <div className="absolute bottom-0 left-0 w-full p-4 space-x-4 bg-opacity-25 bg-black">
          <div className="flex items-center justify-start overflow-x-auto">
            {otherStreams.map((stream, index) => (
              <div
                key={`${index}-video`}
                className="flex-none h-40 w-[200px] p-2">
                <div className="card bordered shadow-lg">
                  <VideoCard
                    stream={stream}
                    details="details"
                    onClick={() => handleStreamClick(stream)}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
