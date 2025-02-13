import { Card, CardContent, CardHeader, Grid2 } from "@mui/material";
import { ReactNode } from "react";
import {Skill, TypeOfSkill} from "../types/Vaardigheid";

export function NavigationCardSkill({
  title,
  subheader,
  skills,
}: {
  title: string;
  subheader: string;
  skills: Record<TypeOfSkill, [Skill, ReactNode][]>
}) {
  return (
      <Card sx={{ minWidth: 300 }}>
        <CardContent aria-label={title} aria-description={subheader}>
          <Grid2 container spacing={1}>
            <Grid2 container size={{xs: 12, lg: 4.8}}>
              <Grid2 container size={12} spacing={1}>
                {skills["Beroeps"].map(([skill, node]) =>
                    <Grid2 key={skill} size={{xs: 12, lg: 6}}>
                      {node}
                    </Grid2>
                )}
              </Grid2>
            </Grid2>
            <Grid2 container size={{xs: 12, lg: 7.2}} spacing={1}>
              <Grid2 container size={12} spacing={1}>
                {skills["Persoonsvormende"].map(([skill, node]) =>
                    <Grid2 key={skill} size={{xs: 12, lg: 4}}>
                      {node}
                    </Grid2>
                )}
              </Grid2>
              <Grid2 container size={12} spacing={1}>
                {skills["Sociale"].map(([skill, node]) =>
                    <Grid2 key={skill} size={{xs: 12, lg: 4}}>
                      {node}
                    </Grid2>
                )}
              </Grid2>
            </Grid2>
          </Grid2>
        </CardContent>
      </Card>
  );
}
