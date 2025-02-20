import { Card, CardContent, CardHeader, Grid2 } from "@mui/material";
import { Level } from "./Level";
import { Niveau } from "../types/Niveau";

export function LevelsCard(props: {
  title: string;
  item: { [key in Niveau]: { title: string; info: string | null } };
}) {
  return (
    <Grid2 size={12}>
      <Card component={"section"}>
        {/* TODO translate titles */}
        <CardHeader title={props.title} component={"h1"} />
        <CardContent>
          <Grid2 container spacing={5}>
            {Object.entries(props.item).map(([niveauKey, level]) => (
              <Level
                key={niveauKey}
                niveauKey={niveauKey}
                title={level.title}
                info={level.info}
              />
            ))}
          </Grid2>
        </CardContent>
      </Card>
    </Grid2>
  );
}
