import { TypeOfSkill, typeOfSkills } from "../types/Vaardigheid";
import { Property } from "csstype";

export function getColour(query_param_value: string, alpha = 1) {
	const ColourMap: Map<TypeOfSkill, Property.Color> = new Map([
		["Beroeps", `rgba(152, 172, 204, ${alpha})`],
		["Persoonsvormende", `rgba(176, 196, 156, ${alpha})`],
		["Sociale", `rgba(232, 172, 140, ${alpha})`],
	]);

	const result = Object.entries(typeOfSkills).find(([, skils]) => {
		// @ts-expect-error TS2345
		return skils.includes(query_param_value);
	});

	if (result === undefined) {
		return "#ffffff";
	} else {
		return ColourMap.get(<"Beroeps" | "Persoonsvormende" | "Sociale">result[0]);
	}
}
