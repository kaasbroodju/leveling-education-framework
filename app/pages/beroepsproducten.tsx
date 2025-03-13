import Head from "next/head";
import { FormattedMessage, useIntl } from "react-intl";
import { Grid2, Stack } from "@mui/material";
import { NavigationCardButton } from "../components/NavigationCardButton";
import { NavigationCard } from "../components/NavigationCard";
import { useRouter } from "next/router";
import { GetStaticProps, InferGetStaticPropsType } from "next";
import { filterBeroepstaken } from "../util/filterBeroepstaken";
import {
	architecture_layers,
	Architectuurlaag,
} from "../types/Architectuurlaag";
import { Activiteit, activities } from "../types/Activiteit";
import DefaultErrorPage from "next/error";
import { getBeroepsproducten } from "../util/getBeroepsproducten";
import { LevelsCardBeroepsproduct } from "../components/LevelsCardBeroepsproduct";
import { BeroepsProduct } from "../types/BeroepsProduct";
import { CreateBeroepsProduct } from "../components/forms/CreateBeroepsProduct";
import { useSession } from "next-auth/react";

export const getStaticProps: GetStaticProps = async (context) => {
	// migrateToNewFileLayout()
	// static site generation
	const beroepstaken = await getBeroepsproducten();

	if (context.locale === "en")
		// disable english translations whilst there are none
		return {
			notFound: true,
		};

	return {
		props: {
			beroepstaken,
		},
		revalidate: 60, // Rebuild the page every 60 seconds
	};
};

export default function Beroepsproducten({
	beroepstaken,
}: InferGetStaticPropsType<typeof getStaticProps>) {
	const intl = useIntl();
	const router = useRouter();
	const { data: session } = useSession();
	const isLoggedIn = !!session;

	const { activiteit, architectuurlaag } = router.query as {
		activiteit?: Activiteit;
		architectuurlaag?: Architectuurlaag;
	};

	if (activiteit && !activities.includes(activiteit as Activiteit))
		return <DefaultErrorPage statusCode={404} />;

	if (
		architectuurlaag &&
		!architecture_layers.includes(architectuurlaag as Architectuurlaag)
	)
		return <DefaultErrorPage statusCode={404} />;

	const filteredBeroepstaken = filterBeroepstaken<BeroepsProduct[]>(
		beroepstaken,
		{
			activiteit,
			architectuurlaag,
		},
	);

	return (
		<>
			<Head>
				<title>{`LEF - ${intl.formatMessage({ id: "PROFESSIONAL_DUTIES" })}`}</title>
			</Head>

			<Stack spacing={2}>
				{/*<Grid2 size={12}>*/}
				{isLoggedIn ? <CreateBeroepsProduct /> : null}

				{/*</Grid2>*/}

				<Grid2 container spacing={2} component={"header"}>
					<Grid2 size={12} component={"section"}>
						<NavigationCard
							title={intl.formatMessage({ id: "ARCHITECTURE_LAYERS" })}
							subheader={intl.formatMessage({
								id: "ARCHITECTURE_LAYERS_SUBHEADER",
							})}
						>
							{architecture_layers.map((architecture_layer) => (
								<NavigationCardButton
									key={architecture_layer}
									title={<FormattedMessage id={architecture_layer} />}
									query_param_key="architectuurlaag"
									query_param_value={architecture_layer}
									props={{ size: { xs: 12, lg: 2.4 } }}
								/>
							))}
						</NavigationCard>
					</Grid2>
					<Grid2 size={12} component={"section"}>
						<NavigationCard
							title={intl.formatMessage({ id: "ACTIVITIES" })}
							subheader={intl.formatMessage({ id: "ACTIVITIES_SUBHEADER" })}
						>
							{activities.map((activity) => (
								<NavigationCardButton
									key={activity}
									title={<FormattedMessage id={activity} />}
									query_param_key="activiteit"
									query_param_value={activity}
									props={{ size: { xs: 12, lg: 2.4 } }}
								/>
							))}
						</NavigationCard>
					</Grid2>
				</Grid2>
				{/*<Grid2 spacing={2} size={12}>*/}
				{Object.entries(filteredBeroepstaken).map(([beroepstaakKey, item]) => (
					<LevelsCardBeroepsproduct
						key={beroepstaakKey}
						title={beroepstaakKey}
						item={item}
					/>
				))}
				{/*</Grid2>*/}
			</Stack>
		</>
	);
}
