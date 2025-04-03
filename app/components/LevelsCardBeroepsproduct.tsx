import { Card, CardContent, CardHeader, Grid2, Stack } from "@mui/material";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsProductBadge } from "./BeroepsProductBadge";

export function LevelsCardBeroepsproduct(props: {
	title: string;
	items: BeroepsProduct[];
}) {
	const numColumns = 3;
	const numItemsInColumn = Math.ceil(props.items.length / numColumns);
	const buckets = Array.from({ length: numColumns }, (_, rowIndex) => {
		return props.items.slice(
			rowIndex * numItemsInColumn,
			(rowIndex + 1) * numItemsInColumn,
		);
	});

	return (
		<Grid2 size={12}>
			<Card component={"section"}>
				{/* TODO translate titles */}
				<CardHeader title={props.title} component={"h1"} />
				<CardContent>
					<Grid2 container columnSpacing={1}>
						{buckets.map((column, colIndex) => (
							<Grid2 size={{ xs: 12, sm: 6, lg: 4 }} key={colIndex}>
								<Stack spacing={1}>
									{column.map((product) => (
										<BeroepsProductBadge key={product.id} product={product} />
									))}
								</Stack>
							</Grid2>
						))}
					</Grid2>
				</CardContent>
			</Card>
		</Grid2>
	);
}
