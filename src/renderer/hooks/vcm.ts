import { useCallback, useEffect, useState } from 'react';
import SimplePeer from 'simple-peer';
import type { Room } from 'rust-tc-sdk';

interface RemoteStreams {
  [id: string]: MediaStream;
}

interface RegisterPeer {
  [nodeId: string]: SimplePeer.Instance;
}

export const useVCM = (room?: Room) => {
  const [peers, setPeers] = useState<RegisterPeer>({});
  const [localStream, setLocalStream] = useState<MediaStream | undefined>();
  const [remoteStreams, setRemoteStreams] = useState<RemoteStreams>({});
  const [videoDevices, setVideoDevices] = useState<MediaDeviceInfo[]>([]);
  const [audioDevices, setAudioDevices] = useState<MediaDeviceInfo[]>([]);
  const [videoDevice, setVideoDevice] = useState<MediaDeviceInfo>();
  const [audioDevice, setAudioDevice] = useState<MediaDeviceInfo>();
  const [isAudioMuted, setIsAudioMuted] = useState(false);
  const [isVideoOn, setIsVideoOn] = useState(true); // State to track video status

  const handleStream = (id: string, stream: MediaStream) => {
    setRemoteStreams((prev) => ({ ...prev, [id]: stream }));
  };

  /* useEffect(() => {
    if (room) {
      room.on('stream', handleStream);
    }
    return () => {
      if (room) {
        room.off('stream', handleStream);
      }
    };
  }, []); */

  // Fetch available video input devices
  useEffect(() => {
    navigator.mediaDevices
      .enumerateDevices()
      .then((devices) => {
        setVideoDevices(
          devices.filter((device) => device.kind === 'videoinput'),
        );
        setAudioDevices(
          devices.filter((device) => device.kind === 'audioinput'),
        );
        return null;
      })
      .catch((err) => console.error('Failed to get devices', err));
  }, []);

  // Initialize and get local video/audio stream.
  useEffect(() => {
    navigator.mediaDevices
      .getUserMedia({ video: true, audio: true })
      .then((stream) => setLocalStream(stream))
      .catch((err) => console.error('Failed to get local stream', err));
  }, []);

  // Change camera based on the provided deviceId
  const changeCamera = useCallback(
    (device: MediaDeviceInfo) => {
      navigator.mediaDevices
        .getUserMedia({
          video: { deviceId: { exact: device.deviceId } },
          audio: true,
        })
        .then((newStream) => {
          if (localStream) {
            const oldTrack = localStream.getVideoTracks()[0];
            if (oldTrack) oldTrack.stop();
          }
          // Stop old track

          // Replace with the new track
          const newTrack = newStream.getVideoTracks()[0];
          if (newTrack) {
            if (localStream) {
              localStream.addTrack(newTrack);
            }
          }
          setLocalStream(newStream);
          setVideoDevice(device);
          return null;
        })
        .catch((err) => console.error('Failed to change camera', err));
    },
    [localStream],
  );

  const toggleVideo = useCallback(() => {
    if (localStream) {
      const videoTrack = localStream.getVideoTracks()[0];
      if (videoTrack) {
        videoTrack.enabled = !videoTrack.enabled;
        setIsVideoOn(videoTrack.enabled);
      }
    }
  }, [localStream]);

  const muteAudio = useCallback(() => {
    if (localStream) {
      const audioTrack = localStream.getAudioTracks()[0];
      if (audioTrack) {
        audioTrack.enabled = !audioTrack.enabled;
        setIsAudioMuted(!audioTrack.enabled);
      }
    }
  }, [localStream]);

  const changeAudioDevice = useCallback(
    (device: MediaDeviceInfo) => {
      navigator.mediaDevices
        .getUserMedia({
          video: true,
          audio: { deviceId: { exact: device.deviceId } },
        })
        .then((newStream) => {
          // Stop old track
          if (localStream) {
            const oldTrack = localStream.getAudioTracks()[0];
            if (oldTrack) oldTrack.stop();
          }

          // Replace with the new track
          const newTrack = newStream.getAudioTracks()[0];
          if (newTrack) {
            if (localStream) {
              localStream.addTrack(newTrack);
            }
          }
          setLocalStream(newStream);
          setAudioDevice(device);
          return null;
        })
        .catch((err) => console.error('Failed to change audio device', err));
    },
    [localStream],
  );

  return {
    changeCamera,
    localStream,
    remoteStreams,
    peers,
    videoDevices,
    audioDevices,
    isAudioMuted,
    muteAudio,
    changeAudioDevice,
    toggleVideo,
    isVideoOn,
    videoDevice,
    audioDevice,
  };
};
