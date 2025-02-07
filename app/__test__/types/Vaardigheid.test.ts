import { skills } from "../../types/Vaardigheid";

describe("Vaardigheid", () => {
  const SKILLS = [
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
  ];
  it("should not change from predefined definitions", () => {
    expect(skills).toEqual(SKILLS);
  });
});
