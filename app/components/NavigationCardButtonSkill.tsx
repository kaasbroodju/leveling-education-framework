import React, { ReactNode } from "react";
import { useRouter } from "next/router";
import {
	Button,
	Grid2,
	Grid2Props,
	useMediaQuery,
	useTheme,
} from "@mui/material";
import Link from "next/link";
import { getIcon } from "../util/vaardighedenToIcon";
import { TypeOfSkill } from "../types/Vaardigheid";
import { Property } from "csstype";

export function NavigationCardButtonSkill({
	title,
	query_param_key,
	query_param_value,
	type_of_skill,
	props = { size: { xs: 12, sm: 6 } },
}: {
	title: string | ReactNode;
	query_param_key: string;
	query_param_value: string;
	type_of_skill: TypeOfSkill;
	props?: Grid2Props;
}) {
	const router = useRouter();
	const theme = useTheme();
	const isLarge = useMediaQuery(theme.breakpoints.up("lg"));

	let href;
	if (router.query[query_param_key] === query_param_value) {
		// eslint-disable-next-line @typescript-eslint/no-unused-vars,unused-imports/no-unused-vars
		const { [query_param_key]: _, ...query } = router.query;
		href = { query };
	} else {
		href = { query: { ...router.query, [query_param_key]: query_param_value } };
	}

	const activeColourMap: Map<TypeOfSkill, Property.Color> = new Map([
		["Beroeps", "rgba(152, 172, 204, 1)"],
		["Persoonsvormende", "rgba(176, 196, 156, 1)"],
		["Sociale", "rgba(232, 172, 140, 1)"],
	]);

	const colourMap: Map<TypeOfSkill, Property.Color> = new Map([
		["Beroeps", "rgba(152, 172, 204, .9)"],
		["Persoonsvormende", "rgba(176, 196, 156, .9)"],
		["Sociale", "rgba(232, 172, 140, .9)"],
	]);

	const colour = colourMap.get(type_of_skill);
	const activeColour = activeColourMap.get(type_of_skill);
	const isActive = query_param_value === router.query[query_param_key];

	const icon = getIcon(query_param_value);

	return (
		<Grid2 {...props}>
			<Link
				href={href}
				style={{ width: "100%", color: theme.palette.primary.main }}
			>
				<Button
					sx={
						isLarge
							? {
									textTransform: "none",
									flexDirection: "column",
									aspectRatio: "1.7777777777777777",
									borderWidth: isActive ? 2 : 0,
									":hover": { borderWidth: 2 },
								}
							: { textTransform: "none" }
					}
					variant={
						// isActive
						//   ? "contained"
						//   : "outlined"
						"outlined"
					}
					fullWidth
					style={{
						backgroundColor: isActive ? activeColour : colour,
						color: theme.palette.primary.main,
					}}
				>
					{isLarge ? icon : null}

					{title}
				</Button>
			</Link>
		</Grid2>
	);
}
