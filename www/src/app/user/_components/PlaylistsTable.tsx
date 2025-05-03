"use client";
import { components } from "@/client/echo";

export type PlaylistTableProps = {
  playlists: Array<components["schemas"]["SimplePlaylist"]>;
};
export default function PlaylistTable({ playlists }: PlaylistTableProps) {
  return (
    <div className="overflow-x-auto mt-8">
      <table className="overflow-hidden min-w-full rounded-lg shadow-lg">
        <thead className="bg-accent text-text-inverse-primary border-b-1">
          <tr>
            <th className="py-5 px-6 font-bold uppercase text-start">Loaded</th>
            <th className="py-5 px-6 font-bold uppercase text-start">Image</th>
            <th className="py-5 px-6 font-bold uppercase text-start">Name</th>
            <th className="py-5 px-6 font-bold uppercase text-start">Tracks</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-accent">
          {playlists.map((playlist) => (
            <tr
              key={playlist.id}
              className="transition-colors hover:text-text-inverse-primary hover:bg-secondary"
            >
              <td className="py-5 px-6">Loaded or not</td>
              <td className="py-5 px-6">
                {playlist.images[0] && (
                  <div className="overflow-hidden relative w-16 h-16 bg-prime">
                    <img
                      src={playlist.images[0].url}
                      alt="Playlist cover"
                      width={playlist.images[0].width || 0}
                      height={playlist.images[0].height || 0}
                    />
                  </div>
                )}
              </td>
              <td className="py-5 px-6">{playlist.name}</td>
              <td className="py-5 px-6">{playlist.total_tracks}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
