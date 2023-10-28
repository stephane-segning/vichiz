import { useEffect, useRef } from 'react';

interface VideoDisplayProps {
  stream?: MediaStream;
}

export function VideoDisplay({ stream }: VideoDisplayProps) {
  const videoRef = useRef<HTMLVideoElement | null>(null);

  useEffect(() => {
    if (videoRef.current && stream) {
      videoRef.current.srcObject = stream;
    }
  }, [stream]);

  return (
    <video
      ref={videoRef}
      className="w-full h-full object-cover rounded-lg"
      autoPlay
      playsInline
    />
  );
}
