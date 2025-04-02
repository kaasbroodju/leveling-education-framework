import { getBeroepstakenOrVaardigheden } from "./getBeroepstakenOrVaardigheden";
import fsPromises from "fs/promises";

export async function migrateToNewFileLayout() {
	const lang: Array<"nl" | "en"> = ["nl", "en"];
	const typeOfSkill: Array<"hboi" | "vaardigheden"> = ["hboi", "vaardigheden"];

	for (const language of lang) {
		for (const type of typeOfSkill) {
			const source = await getBeroepstakenOrVaardigheden(type, language);

			for (const key of Object.keys(source)) {
				const taken = source[key];

				const activiteit = Object.keys(taken);

				for (const level of activiteit) {
					await fsPromises.mkdir(`datav2/${type}/${language}/${key}/${level}`, {
						recursive: true,
					});
					const taak = taken[level];
					await fsPromises.writeFile(
						`datav2/${type}/${language}/${key}/${level}/description.txt`,
						taak.title,
					);
					if (taak.info !== "") {
						await fsPromises.writeFile(
							`datav2/${type}/${language}/${key}/${level}/info.txt`,
							taak.info,
						);
					}

					if (type === "hboi") {
						await fsPromises.mkdir(`datav2/examples/${key}/${level}`, {
							recursive: true,
						});
						await fsPromises.writeFile(
							`datav2/examples/${key}/${level}.json`,
							"[]",
							"utf-8",
						);
					}
				}
			}
		}
	}
}
