import { ipcMain } from 'electron';
import { tcp } from '@libp2p/tcp';
// import tcp from '@libp2p/tcp/dist/src/index.js';
import { createLibp2p, Libp2p, Libp2pOptions } from 'libp2p';
import { mplex } from '@libp2p/mplex';
import { noise } from '@libp2p/noise';
import { mdns } from '@libp2p/mdns';
import { yamux } from '@chainsafe/libp2p-yamux';
import Multimap from 'multimap';
import { PeerInfo } from '@libp2p/interface/peer-info';
import { webSockets } from '@libp2p/websockets';
import { pipe } from 'it-pipe';
import * as pako from 'pako';
import { Stream } from 'stream';

interface Room {
  id: string;
  name: string;
  secretToken: string;
}

const toOptions = (room: Room): Libp2pOptions => ({
  transports: [tcp(), webSockets()],
  streamMuxers: [yamux(), mplex()],
  connectionEncryption: [noise()],
  peerDiscovery: [
    mdns({
      serviceTag: `__p2p_.${room.id}._tcp.ssegning`,
    }),
  ],
  addresses: {
    listen: ['/ip4/0.0.0.0/tcp/0', '/ip4/0.0.0.0/tcp/32023/ws'],
  },
});

const networks: Record<string, Libp2p> = {};
const peers = new Multimap<string, PeerInfo>();

const cancel = async (roomId: string) => {
  networks[roomId].stop();
  delete networks[roomId];
};
ipcMain.on('net-create', async (event, room: Room) => {
  const network = await createLibp2p(toOptions(room));

  await network.handle('/ssegning/1.0.0', async ({ stream }) => {
    await pipe(stream, async function (source) {
      for await (const msg of source) {
        try {
          const { type, data } = JSON.parse(
            pako.inflate(msg.subarray(), {
              to: 'string',
            }),
          );

          ipcMain.emit(`net-message-${room.id}`, type, data);
        } catch (e) {
          console.error(e);
        }
      }
    });
  });

  network.addEventListener('peer:discovery', (peer) => {
    peers.set(room.id, peer.detail);
  });

  network.addEventListener('peer:connect', async (evt) => {
    const peerId = evt.detail;
    const peer = peers.get(room.id).find((p) => p.id === peerId)!;

    const is = new Stream.Readable();
    const os = await network.dialProtocol(peerId, '/ssegning/1.0.0');

    await pipe(is, os);

    ipcMain.emit(
      `net-node-available-${room.id}`,
      peer.id,
      async (type: string, data: any) => {
        is.push(pako.deflate(JSON.stringify({ type, data })));
      },
    );

    ipcMain.on(
      `net-node-unavailable-${room.id}`,
      async (_, unavailablePeerId) => {
        if (peerId === unavailablePeerId) {
          is.destroy();
        }
      },
    );
  });

  network.addEventListener('peer:disconnect', async (evt) => {
    const peerId = evt.detail;
    await network.hangUp(peerId);
    ipcMain.emit(`net-node-unavailable-${room.id}`, peerId);
  });

  await network.start();
  networks[room.id] = network;

  ipcMain.on('net-broadcast', async (event, roomId, type, data) => {
    await network.dialProtocol(peers.get(roomId)[0].id, '/ssegning/1.0.0');

    event.reply(`net-broadcast-${roomId}`, true);
  });

  event.reply('net-create', network.peerId);
});

ipcMain.on('net-destroy', async (event, roomId) => {
  await cancel(roomId);
  event.reply('net-destroy', true);
});
