import { Card, CardContent, Grid2 } from "@mui/material";
import { ReactNode } from "react";

export function NavigationCard({
	title,
	subheader,
	children,
}: {
	title: string;
	subheader: string;
	children: ReactNode;
}) {
	return (
		<Card sx={{ minWidth: 300 }}>
			<CardContent aria-label={title} aria-description={subheader}>
				<Grid2 container spacing={1}>
					{children}
				</Grid2>
			</CardContent>
		</Card>
	);
}
