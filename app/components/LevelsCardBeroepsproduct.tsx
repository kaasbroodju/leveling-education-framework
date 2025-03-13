import { Card, CardContent, CardHeader, Grid2 } from "@mui/material";
import { Niveau } from "../types/Niveau";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsLevel } from "./BeroepsLevel";

export function LevelsCardBeroepsproduct(props: {
	title: string;
	item: { [key in Niveau]: BeroepsProduct[] };
}) {
	return (
		<Grid2 size={12}>
			<Card component={"section"}>
				{/* TODO translate titles */}
				<CardHeader title={props.title} component={"h1"} />
				<CardContent>
					<Grid2 container spacing={5}>
						{Object.entries(props.item).map(([niveauKey, products]) => (
							<BeroepsLevel
								key={niveauKey}
								niveauKey={niveauKey}
								products={products}
								title={niveauKey}
							/>
						))}
					</Grid2>
				</CardContent>
			</Card>
		</Grid2>
	);
}
