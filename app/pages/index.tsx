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
import { Grid2 } from "@mui/material";
import {migrateToNewFileLayout} from "../util/migrateToNewFileLayout";
import {getVaardigheden} from "../util/getVaardigheden";
import {NavigationCardSkill} from "../components/NavigationCardSkill";
import {ReactNode} from "react";

export const getStaticProps: GetStaticProps = async (context) => {
  // static site generation
  const vaardigheden = await getVaardigheden(
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
  // migrateToNewFileLayout();
  const intl = useIntl();
  const router = useRouter();

  const { vaardigheid } = router.query as {
   vaardigheid?: Skill
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
            props={{size: { xs: 12 }}}
        />]
    ),
    Persoonsvormende: typeOfSkills["Persoonsvormende"].map((a) =>
        [a, <NavigationCardButtonSkill
            key={a}
            title={<FormattedMessage id={a} />}
            type_of_skill={"Persoonsvormende"}
            query_param_key="vaardigheid"
            query_param_value={a}
            props={{size: { xs: 12 }}}
        />]
    ),
    Sociale: typeOfSkills["Sociale"].map((a) =>
        [a, <NavigationCardButtonSkill
            key={a}
            title={<FormattedMessage id={a} />}
            type_of_skill={"Sociale"}
            query_param_key="vaardigheid"
            query_param_value={a}
            props={{size: { xs: 12 }}}
        />]
    ),
  }

  return (
    <>
      <Head>
        <title>{`LEF - ${intl.formatMessage({ id: "SKILLS" })}`}</title>
      </Head>
      <Grid2 container spacing={2}>
        <Grid2 size={12} component={"header"}>
          <NavigationCardSkill
            title={intl.formatMessage({ id: "SKILLS"}) }
            subheader={intl.formatMessage({ id: "SKILLS_SUBHEADER"}) }
            skills={skillsMap}
          />
        </Grid2>
        {Object.entries(filteredVaardigheden).map(([vaardighedenKey, vaardigheid]) => (
          <LevelsCard
            key={vaardighedenKey}
            title={vaardighedenKey}
            item={vaardigheid}
          />
        ))}
      </Grid2>
    </>
  );
}
