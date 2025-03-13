import { Skill, SkillLevels } from "../types/Vaardigheid";
import { Niveau } from "../types/Niveau";
import { db } from "../lib/db";

export async function getVaardigheden(
	locale?: "nl" | "en",
): Promise<SkillLevels> {
	return (await db.skillDescription.findMany()).reduce(
		(acc, vaardigheid) => {
			if (!acc[vaardigheid.skillId as Skill]) {
				acc[vaardigheid.skillId as Skill] = {};
			}

			// @ts-ignore
			acc[vaardigheid.skillId as Skill][`${vaardigheid.level}` as Niveau] = {
				title: vaardigheid.description,
				info: vaardigheid.sublament,
			};
			return acc;
		},
		{} as Partial<{
			[key in Skill]: Partial<{
				[key in Niveau]: { title: string; info: string | null };
			}>;
		}>,
	) as SkillLevels;
}
