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
	BIM: "#9A7300",
	CSC: "#006400",
	FE: "#D35400",
	"UI/UX": "#880E4F",
	TI: "#001F3F",
	GD: "#8950C7",
} as { [key in Guild]: string };
