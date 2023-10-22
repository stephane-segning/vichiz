import { useCallback, useEffect, useState } from 'react';
import SimplePeer from 'simple-peer';
import { Network } from 'ataraxia/src/Network';

interface RemoteStreams {
  [id: string]: MediaStream;
}

interface RegisterPeer {
  [nodeId: string]: SimplePeer.Instance;
}

export const useVCM = (net: Network | null) => {
  const [peers, setPeers] = useState<RegisterPeer>({});
  const [localStream, setLocalStream] = useState<MediaStream | undefined>();
  const [remoteStreams, setRemoteStreams] = useState<RemoteStreams>({});
  const [videoDevices, setVideoDevices] = useState<MediaDeviceInfo[]>([]);
  const [audioDevices, setAudioDevices] = useState<MediaDeviceInfo[]>([]);
  const [isAudioMuted, setIsAudioMuted] = useState(false);
  const [isVideoOn, setIsVideoOn] = useState(true); // State to track video status

  const handleStream = (id: string, stream: MediaStream) => {
    setRemoteStreams((prev) => ({ ...prev, [id]: stream }));
  };

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
    (deviceId: string) => {
      if (!localStream) return;

      navigator.mediaDevices
        .getUserMedia({
          video: { deviceId: { exact: deviceId } },
          audio: true,
        })
        .then((newStream) => {
          // Stop old track
          const oldTrack = localStream.getVideoTracks()[0];
          if (oldTrack) oldTrack.stop();

          // Replace with the new track
          const newTrack = newStream.getVideoTracks()[0];
          if (newTrack) {
            localStream.addTrack(newTrack);
            setLocalStream(localStream);
          }
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
    (deviceId: string) => {
      if (!localStream) return;

      navigator.mediaDevices
        .getUserMedia({
          video: true,
          audio: { deviceId: { exact: deviceId } },
        })
        .then((newStream) => {
          // Stop old track
          const oldTrack = localStream.getAudioTracks()[0];
          if (oldTrack) oldTrack.stop();

          // Replace with the new track
          const newTrack = newStream.getAudioTracks()[0];
          if (newTrack) {
            localStream.addTrack(newTrack);
            setLocalStream(localStream);
          }
          return null;
        })
        .catch((err) => console.error('Failed to change audio device', err));
    },
    [localStream],
  );

  useEffect(() => {
    if (!net) return () => {};

    // Handle the incoming stream from a peer.
    net.onNodeAvailable((node) => {
      const simplePeer = new SimplePeer({
        stream: localStream,
      });

      simplePeer.on('stream', (stream: MediaStream) => {
        handleStream(node.id, stream);
      });

      simplePeer.on('signal', async (data) => {
        await node.send('WEBRTC_SIGNAL', data);
      });

      node.onMessage((msg) => {
        if (msg.type === 'WEBRTC_SIGNAL') {
          simplePeer.signal(msg.data);
        }
      });

      setPeers((prev) => ({ ...prev, [node.id]: simplePeer }));
    });

    net.onNodeUnavailable(async (node) => {
      peers[node.id].destroy();
    });

    return () => {
      // Cleanup connections if the hook is unmounted
      for (const stream of Object.values(remoteStreams)) {
        stream.getTracks().forEach((track) => track.stop());
      }
      if (localStream) {
        localStream.getTracks().forEach((track) => track.stop());
      }
    };
  }, [localStream, net, peers, remoteStreams]);

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
  };
};
