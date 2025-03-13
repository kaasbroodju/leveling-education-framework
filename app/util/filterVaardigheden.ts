import { Skill, SkillLevels, skills } from "../types/Vaardigheid";

export function filterVaardigheden(
	vaardigheden: SkillLevels,
	{ vaardigheid }: { vaardigheid?: Skill },
): Partial<SkillLevels> {
	if (vaardigheid && skills.includes(vaardigheid as Skill)) {
		return Object.fromEntries([[vaardigheid, vaardigheden[vaardigheid]]]);
	} else {
		return vaardigheden;
	}
}
