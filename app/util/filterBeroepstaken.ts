import { Architectuurlaag } from "../types/Architectuurlaag";
import { Activiteit } from "../types/Activiteit";
import { Niveau } from "../types/Niveau";
import { HBOILevels } from "../types/HBOI";

export function filterBeroepstaken<T>(
  beroepstaken: HBOILevels<T>,
  {
    architectuurlaag,
    activiteit,
  }: {
    architectuurlaag?: Architectuurlaag;
    activiteit?: Activiteit;
  },
): Partial<HBOILevels<T>> {
  // Filter beroepstaken based on query paramaters
  let filteredBeroepstaken = beroepstaken as Partial<{
    [key in `${Architectuurlaag} ${Activiteit}`]: { [key in Niveau]: T };
  }>;
  if (architectuurlaag) {
    filteredBeroepstaken = Object.fromEntries(
      Object.entries(filteredBeroepstaken).filter(([key]) =>
        key.includes(architectuurlaag),
      ),
    );
  }

  if (activiteit) {
    filteredBeroepstaken = Object.fromEntries(
      Object.entries(filteredBeroepstaken).filter(([key]) =>
        key.includes(activiteit),
      ),
    );
  }

  return filteredBeroepstaken;
}
