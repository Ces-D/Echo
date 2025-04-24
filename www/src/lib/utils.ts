import type { Metadata } from "next";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/** Combine class names */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/** Standard for app metadata */
export function pageMetadata(
  param: Pick<Metadata, "title" | "description">,
): Metadata {
  return {
    title: typeof param.title === "string" ? `${param.title} | Echo` : "Echo",
    keywords: [
      "music organizer",
      "playlist manager",
      "Spotify sync",
      "music queue",
      "playlist editor",
      "music library",
      "Spotify app",
      "playlist tool",
      "auto playlists",
      "music sorter",
      "queue manager",
      "playlist sync",
      "music curation",
      "playlist maker",
      "smart playlists",
      "Spotify manager",
      "dynamic playlists",
      "music updater",
      "playlist cleanup",
      "music discovery",
    ],

    description: param.description ?? "A music playlist organizer",
    authors: [{ name: "Cesar Diaz", url: "https://github.com/Ces-D/Echo" }],
    applicationName: "Echo",
  };
}
