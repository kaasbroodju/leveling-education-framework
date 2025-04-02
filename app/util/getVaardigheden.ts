import path from "path";
import fsPromises from "fs/promises";
import { SkillLevels, skills } from "../types/Vaardigheid";
import * as fs from "node:fs";
import { Niveau, niveaus } from "../types/Niveau";

export async function getVaardigheden(
	locale?: "nl" | "en",
): Promise<SkillLevels> {
	const output: Partial<SkillLevels> = {};

	if (!locale) locale = "nl";

	for (const skill of skills) {
		// for (let activity of activities) {

		const filePath = path.join(
			process.cwd(),
			`datav2/vaardigheden/${locale}/${skill}`,
		);

		const beroepspoduct: Partial<{
			[key in Niveau]: { title: string; info: string | null };
		}> = {};

		for (const level of niveaus) {
			const description = await fsPromises.readFile(
				path.join(filePath, level, "description.txt"),
				"utf-8",
			);

			let info: null | string = null;

			if (fs.existsSync(path.join(filePath, level, "info.txt"))) {
				info = await fsPromises.readFile(
					path.join(filePath, level, "info.txt"),
					"utf-8",
				);
			}

			beroepspoduct[level] = {
				title: description,

				info,
			};
		}

		output[skill] = beroepspoduct as {
			[key in Niveau]: { title: string; info: string | null };
		};
	}

	return output as SkillLevels;
}
