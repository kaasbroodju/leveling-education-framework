import {Card, CardContent, CardHeader, Grid2, Stack} from "@mui/material";
import { Niveau } from "../types/Niveau";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsLevel } from "./BeroepsLevel";
import {BeroepsProductBadge} from "./BeroepsProductBadge";

export function LevelsCardBeroepsproduct(props: {
	title: string;
	items: BeroepsProduct[];
}) {
	const numColumns = 3; // We want 3 columns
	const numItemsInColumn = Math.ceil(props.items.length / numColumns);
	console.log(props.items.length);
	// Step 1: Arrange in row-major order
	const buckets = Array.from({ length: numColumns }, (_, rowIndex) => {
		console.log(rowIndex, rowIndex * numItemsInColumn, (rowIndex + 1) * numItemsInColumn);
		return props.items.slice(
			rowIndex * numItemsInColumn,
			(rowIndex + 1) * numItemsInColumn,
		);
	});

	console.log(buckets)
	return (
		<Grid2 size={12}>
			<Card component={"section"}>
				{/* TODO translate titles */}
				<CardHeader title={props.title} component={"h1"} />
				<CardContent>
					{/*<Grid2 container spacing={5}>*/}
					{/*	{props.items.map((product) => (*/}
					{/*		<BeroepsProductBadge key={product.id} product={product} />*/}
					{/*	))}*/}
					{/*</Grid2>*/}
					<Grid2 container columnSpacing={1}>
						{/*{columns.map((column, columnIndex) => (*/}
						{/*	<Grid2 size={{ xs: 12, sm: 6, lg: 4 }} key={columnIndex}>*/}
						{/*		{column.map((product) => (*/}
						{/*			<BeroepsProductBadge key={product.id} product={product} />*/}
						{/*		))}*/}
						{/*	</Grid2>*/}
						{/*))}*/}
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
