// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from "next";
import { BeroepstaakOrVaardigheid } from "../../../../types/BeroepstakenOrVaardigheden";
import { getBeroepstakenOrVaardigheden } from "../../../../util/getBeroepstakenOrVaardigheden";
import {getVaardigheden} from "../../../../util/getVaardigheden";
import {Skill, SkillLevels} from "../../../../types/Vaardigheid";
import {Niveau} from "../../../../types/Niveau";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<{
    [key in Niveau]: {
      title: string;
      info: string | null;
    }
  } | { error: string }>
) {
  const { vaardigheid } = req.query as { vaardigheid: Skill };
  // const locale = req.headers["accept-language"]?.startsWith("en") ? "en" : "nl";
  //
  // if (locale === "en")
  //   // disable english translations whilst there are none
  //   return res.status(501).json({ error: "Locale not implemented yet" });

  const vaardigheden = await getVaardigheden(
    "nl"
  );

  // if (!(vaardigheid in vaardigheden)) {
  //   res.status(404).json({ error: `Vaardigheid: ${vaardigheid} not found` });
  // }

  res.status(200).json(vaardigheden[vaardigheid]);
}
