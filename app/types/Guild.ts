export const guilds = [
	"AI",
	"BE",
	"BIM",
	"CSC",
	"FE",
	"UI/UX",
	"TI",
	"GD",
] as const;

export type Guild = (typeof guilds)[number];

export const guild_colours = {
	AI: "#4B0082",
	BE: "#B71C1C",
	BIM: "#005F5F",
	CSC: "#006400",
	FE: "#D35400",
	"UI/UX": "#880E4F",
	TI: "#001F3F",
	GD: "#9A7300",
} as { [key in Guild]: string };
