import { Niveau } from "./Niveau";

export const skills = [
  "Overzicht creëren",
  "Kritisch oordelen",
  "Juiste kennis ontwikkelen",
  "Kwalitatief product maken",
  "Plannen",
  "Boodschap delen",
  "Samenwerken",
  "Flexibel opstellen",
  "Pro-actief handelen",
  "Reflecteren",
] as const;

export type Skill = (typeof skills)[number];
export type TypeOfSkill = "Beroeps" | "Persoonsvormende" | "Sociale";

export const typeOfSkills: Record<TypeOfSkill, ReadonlyArray<Skill>> = {
  Beroeps: [skills[0], skills[1], skills[2], skills[3]],
  Persoonsvormende: [skills[4], skills[5], skills[6]],
  Sociale: [skills[7], skills[8], skills[9]],
} as const;

export type SkillLevels = {
  [key in Skill]: {
    [key in Niveau]: {
      title: string;
      info: string | null;
    };
  };
};
