import Head from "next/head";
import { FormattedMessage, useIntl } from "react-intl";
import { Grid2 } from "@mui/material";
import { NavigationCardButton } from "../components/NavigationCardButton";
import { NavigationCard } from "../components/NavigationCard";
import { useRouter } from "next/router";
import { GetStaticProps, InferGetStaticPropsType } from "next";
import { BeroepstakenOrVaardigheden as BeroepstakenType } from "../types/BeroepstakenOrVaardigheden";
import { getBeroepstakenOrVaardigheden } from "../util/getBeroepstakenOrVaardigheden";
import { filterBeroepstaken } from "../util/filterBeroepstaken";
import { LevelsCard } from "../components/LevelsCard";
import {
  architecture_layers,
  Architectuurlaag,
} from "../types/Architectuurlaag";
import { Activiteit, activities } from "../types/Activiteit";
import DefaultErrorPage from "next/error";
import {getBeroepstaken} from "../util/getBeroepstaken";
import {getBeroepsproducten} from "../util/getBeroepsproducten";
import {LevelsCardBeroepsproduct} from "../components/LevelsCardBeroepsproduct";
import {migrateToNewFileLayout} from "../util/migrateToNewFileLayout";
import {BeroepsProduct} from "../types/BeroepsProduct";
import {Niveau} from "../types/Niveau";

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
  };
};

export default function Beroepsproducten({
  beroepstaken,
}: InferGetStaticPropsType<typeof getStaticProps>) {
  const intl = useIntl();
  const router = useRouter();

  const { activiteit, architectuurlaag } = router.query as {
    activiteit?: Activiteit, architectuurlaag?: Architectuurlaag
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
    }
  );

  return (
    <>
      <Head>
        <title>{`LEF - ${intl.formatMessage({ id: "PROFESSIONAL_DUTIES" })}`}</title>
      </Head>
      <Grid2 container spacing={2}>
        <Grid2 container spacing={2} component={"header"}>
          <Grid2 size={12} component={"section"}>
            <NavigationCard
                title={intl.formatMessage({id: "ARCHITECTURE_LAYERS" })}
                subheader={intl.formatMessage({id: "ARCHITECTURE_LAYERS_SUBHEADER" })}
            >
              {architecture_layers.map((architecture_layer) => (
                  <NavigationCardButton
                      key={architecture_layer}
                      title={<FormattedMessage id={architecture_layer} />}
                      query_param_key="architectuurlaag"
                      query_param_value={architecture_layer}
                      props={{size: { xs: 12, lg: 2.4 }}}
                  />
              ))}
            </NavigationCard>
          </Grid2>
          <Grid2 size={12} component={"section"}>
            <NavigationCard
                title={intl.formatMessage({id: "ACTIVITIES" })}
                subheader={intl.formatMessage({id: "ACTIVITIES_SUBHEADER" })}
            >
              {activities.map((activity) => (
                  <NavigationCardButton
                      key={activity}
                      title={<FormattedMessage id={activity} />}
                      query_param_key="activiteit"
                      query_param_value={activity}
                      props={{size: { xs: 12, lg: 2.4 }}}
                  />
              ))}
            </NavigationCard>
          </Grid2>
        </Grid2>
        <Grid2 container spacing={2}>
          {Object.entries(filteredBeroepstaken).map(([beroepstaakKey, item]) => (
              <LevelsCardBeroepsproduct
                  key={beroepstaakKey}
                  title={beroepstaakKey}
                  item={item}
              />
          ))}
        </Grid2>
      </Grid2>
    </>
  );
}
