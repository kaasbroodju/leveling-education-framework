import Head from "next/head";
import { FormattedMessage, useIntl } from "react-intl";
import { GetStaticProps, InferGetStaticPropsType } from "next";
import { getBeroepstakenOrVaardigheden } from "../util/getBeroepstakenOrVaardigheden";
import { useRouter } from "next/router";
import DefaultErrorPage from "next/error";
import { LevelsCard } from "../components/LevelsCard";
import {Skill, skills, TypeOfSkill, typeOfSkills} from "../types/Vaardigheid";
import { filterVaardigheden } from "../util/filterVaardigheden";
import { NavigationCardButtonSkill } from "../components/NavigationCardButtonSkill";
import { NavigationCard } from "../components/NavigationCard";
import { Grid } from "@mui/material";
import {NavigationCardSkill} from "../components/NavigationCardSkill";
import {ReactNode} from "react";

export const getStaticProps: GetStaticProps = async (context) => {
  // static site generation
  const vaardigheden = await getBeroepstakenOrVaardigheden(
    "vaardigheden",
    context.locale === "en" ? "en" : "nl"
  );

  if (context.locale === "en")
    // disable english translation whilst there is none
    return {
      notFound: true,
    };

  return {
    props: {
      vaardigheden,
    },
  };
};

export default function Index({
  vaardigheden,
}: InferGetStaticPropsType<typeof getStaticProps>) {
  const intl = useIntl();
  const router = useRouter();

  const { vaardigheid } = router.query as {
    [key: string]: string;
  };

  if (vaardigheid && !skills.includes(vaardigheid as Skill))
    return <DefaultErrorPage statusCode={404} />;

  const filteredVaardigheden = filterVaardigheden(vaardigheden, {
    vaardigheid,
  });


  let skillsMap: Record<TypeOfSkill, Array<[Skill, ReactNode]>> = {
    Beroeps: typeOfSkills["Beroeps"].map((a) =>
        [a, <NavigationCardButtonSkill
            key={a}
            title={<FormattedMessage id={a} />}
            type_of_skill={"Beroeps"}
            query_param_key="vaardigheid"
            query_param_value={a}
            props={{ xs: 12 }}
        />]
    ),
    Persoonsvormende: typeOfSkills["Persoonsvormende"].map((a) =>
        [a, <NavigationCardButtonSkill
            key={a}
            title={<FormattedMessage id={a} />}
            type_of_skill={"Persoonsvormende"}
            query_param_key="vaardigheid"
            query_param_value={a}
            props={{ xs: 12 }}
        />]
    ),
    Sociale: typeOfSkills["Sociale"].map((a) =>
        [a, <NavigationCardButtonSkill
            key={a}
            title={<FormattedMessage id={a} />}
            type_of_skill={"Sociale"}
            query_param_key="vaardigheid"
            query_param_value={a}
            props={{ xs: 12 }}
        />]
    ),
  }

  return (
    <>
      <Head>
        <title>LEF - {intl.formatMessage({ id: "SKILLS" })}</title>
      </Head>
      <Grid container spacing={2}>
        <Grid item xs={12} component={"header"}>
          <NavigationCardSkill
            title={intl.formatMessage({ id: "SKILLS"}) }
            subheader={intl.formatMessage({ id: "SKILLS_SUBHEADER"}) }
            skills={skillsMap}
          />
        </Grid>
        {Object.keys(filteredVaardigheden).map((vaardighedenKey) => (
          <LevelsCard
            key={vaardighedenKey}
            title={vaardighedenKey}
            item={filteredVaardigheden[vaardighedenKey]}
          />
        ))}
      </Grid>
    </>
  );
}
