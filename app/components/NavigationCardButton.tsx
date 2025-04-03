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

export function NavigationCardButton({
	title,
	query_param_key,
	query_param_value,
	props = { size: { xs: 12, sm: 6 } },
}: {
	title: string | ReactNode;
	query_param_key: string;
	query_param_value: string;
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

	const icon = getIcon(query_param_value);

	return (
		<Grid2 {...props}>
			<Link href={href} style={{ width: "100%" }}>
				<Button
					sx={
						isLarge
							? {
									textTransform: "none",
									flexDirection: "column",
									aspectRatio: "1.7777777777777777",
								}
							: { textTransform: "none" }
					}
					variant={
						query_param_value === router.query[query_param_key]
							? "contained"
							: "outlined"
					}
					fullWidth
				>
					{isLarge ? icon : null}

					{title}
				</Button>
			</Link>
		</Grid2>
	);
}
