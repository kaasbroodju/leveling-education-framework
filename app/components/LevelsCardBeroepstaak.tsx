import {
	Box,
	Card,
	CardContent,
	CardHeader,
	Grid2,
	Stack,
	Typography,
	useTheme,
} from "@mui/material";
import { Level } from "./Level";
import { Niveau } from "../types/Niveau";
import { getIcon } from "../util/vaardighedenToIcon";

export function LevelsCardBeroepstaak(props: {
	title: string;
	item: { [key in Niveau]: { title: string; info: string | null } };
}) {
	const index = props.title.indexOf(" ");

	let laag, activiteit;
	if (index !== -1) {
		laag = props.title.slice(0, index);
		activiteit = props.title.slice(index + 1);
	} else {
		laag = props.title;
		activiteit = "";
	}

	const laagIcon = getIcon(laag, 32);
	const activiteitIcon = getIcon(activiteit, 32);
	const theme = useTheme();
	const colour = theme.palette.primary.main;
	return (
		<Grid2 size={12}>
			<Card component={"section"}>
				{/* TODO translate titles */}
				<CardHeader
					title={
						<Stack spacing={2} direction={"row"}>
							<Typography variant={"h5"} component={"h1"}>
								{props.title}
							</Typography>
							{laagIcon}
							{activiteitIcon}
						</Stack>
					}
				/>
				<Box
					bgcolor={colour}
					height={"1rem"}
					borderRadius={"1rem"}
					marginLeft={"16px"}
					marginRight={"16px"}
				/>
				<CardContent>
					<Grid2 container spacing={5}>
						{Object.entries(props.item).map(([niveauKey, level]) => (
							<Level
								key={niveauKey}
								niveauKey={niveauKey}
								title={level.title}
								info={level.info}
							/>
						))}
					</Grid2>
				</CardContent>
			</Card>
		</Grid2>
	);
}
