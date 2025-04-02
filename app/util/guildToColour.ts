import { Guild, guild_colours } from "../types/Guild";

export function guildToColour(guild: Guild): string {
	return guild_colours[guild] ?? "black";
}