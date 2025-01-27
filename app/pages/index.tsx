import Head from "next/head";
import { FormattedMessage, useIntl } from "react-intl";
import { GetStaticProps, InferGetStaticPropsType } from "next";
import { getBeroepstakenOrVaardigheden } from "../util/getBeroepstakenOrVaardigheden";
import { useRouter } from "next/router";
import DefaultErrorPage from "next/error";
import { LevelsCard } from "../components/LevelsCard";
import {Skill, skills, typeOfSkills} from "../types/Vaardigheid";
import { filterVaardigheden } from "../util/filterVaardigheden";
import { NavigationCardButtonSkill } from "../components/NavigationCardButtonSkill";
import { NavigationCard } from "../components/NavigationCard";
import { Grid } from "@mui/material";
import {NavigationCardSkill} from "../components/NavigationCardSkill";

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
            skills={Object.fromEntries(
                Object.entries(typeOfSkills)
                    .map(([typeOfSkill, associatedSkills]) => [typeOfSkill, associatedSkills.map((skill) =>
                <NavigationCardButtonSkill
                    key={skill}
                    title={<FormattedMessage id={skill} />}
                    type_of_skill={typeOfSkill}
                    query_param_key="vaardigheid"
                    query_param_value={skill}
                    props={{ xs: 12 }}
                />
            )]))}
          >
          </NavigationCardSkill>
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
