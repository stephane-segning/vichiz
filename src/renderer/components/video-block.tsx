// VideoCard.tsx
import React from 'react';
import { VideoDisplay } from './video-display';

interface VideoCardProps {
  stream?: MediaStream;
  onClick: () => void;
  details: string;
}

function VideoCard({ stream, onClick, details }: VideoCardProps) {
  return (
    <div
      onClick={onClick}
      className="relative w-full h-full cursor-pointer group">
      <VideoDisplay stream={stream} />

      {/* Overlay */}
      <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300">
        <span className="text-white">{details}</span>
      </div>
    </div>
  );
}

export default VideoCard;
