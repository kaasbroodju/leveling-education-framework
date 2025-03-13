import { Grid2, Stack, Typography } from "@mui/material";
import { FormattedMessage } from "react-intl";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { BeroepsProductBadge } from "./BeroepsProductBadge";

export function BeroepsLevel(props: {
	niveauKey: string;
	products: BeroepsProduct[];
	title: string;
}) {
	return (
		<Grid2 size={{ xs: 12, sm: 6, md: 3 }}>
			<Stack gap={1}>
				<Stack
					direction="row"
					alignItems="center"
					justifyContent="space-between"
				>
					<Typography variant="h6" fontWeight="normal" component={"h2"}>
						<FormattedMessage id="NIVEAU" /> {props.niveauKey}
					</Typography>

					{/*{props.info ? (*/}
					{/*  <InfoDrawerButton niveau={props.niveauKey} info={props.info} />*/}
					{/*) : null}*/}
				</Stack>
				{props.products.map((product) => (
					<BeroepsProductBadge key={product.id} product={product} />
				))}
				{/*<BeroepsProduct product={prop}/>*/}
				{/*<Typography*/}
				{/*  variant="body2"*/}
				{/*  whiteSpace="pre-wrap"*/}
				{/*  sx={{ wordBreak: "break-word" }}*/}
				{/*>*/}
				{/*  {props.title}*/}
				{/*</Typography>*/}
			</Stack>
		</Grid2>
	);
}
